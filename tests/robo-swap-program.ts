import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { assert, expect } from "chai";
import { RoboSwapProgram } from "../target/types/robo_swap_program";
import _ from "lodash";
import winston from "winston";

const winstonFormat = winston.format.printf((info) => 
  `${new Date().toISOString()}-${info.level}: ${JSON.stringify(info.message, null, 4)}`
)
const logger = winston.createLogger({
  transports: [
    new winston.transports.File({ filename: '../log.log', format: winstonFormat })
  ],
})

describe("robo-swap-program", () => {
  
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const program = anchor.workspace.RoboSwapProgram as Program<RoboSwapProgram>;

  let robber: PublicKey
  let robberPDA: PublicKey

  before(async () => {
    robber = provider.wallet.publicKey
    await airdrop(robber, provider)
    robberPDA = await initialize(robber, program)

    logger.info('----------------------------------------')
  })
  after(async() => {
    await program.methods
      .delete()
      .accounts({
        receiver: robber,
      })
  })

    it("Is initialized!", async () => {
        const robots = (await program.account.game.fetch(robberPDA)).robots;
      _.range(26).map(d => {
        const robot = robots[d]
        expect(robot.wallet.toBase58()).to.equal(robber.toBase58())
        expect(robot.owner.toBase58()).to.equal(robber.toBase58())
        expect(robot.idx).to.equal(d)
        expect(robot.ownerIdx).to.equal(d)
        expect(robot.swaps).to.equal(0)
      })
    });
  
    it("Swap!", async () => {
  
      const keypair = Keypair.generate()
      await createAccount(keypair, provider)
      const victim = keypair.publicKey
      const victimPDA = await initialize(victim, program, keypair)
  
      const robberIdx = 3;
      const victimIdx = 8;
  
      try {
        await program.methods
          .swap(victimIdx, robberIdx)
          .accounts({
            robber,
            robberPda: robberPDA,
            victim,
            victimPda: victimPDA,
          })
          .rpc();
      }
      catch (err) {
        console.error(err)
        //assert(false)
      }

      let r = (await program.account.game.fetch(robberPDA)).robots[robberIdx]
      let v = (await program.account.game.fetch(victimPDA)).robots[victimIdx]
      expect(v.ownerIdx).to.equal(robberIdx)
      expect(r.ownerIdx).to.equal(victimIdx)
      expect(v.owner.toBase58()).to.equal(robber.toBase58())
      expect(r.owner.toBase58()).to.equal(victim.toBase58())
  
      await program.methods.delete().accounts({receiver: victim, pda: victimPDA}).rpc()
    });
  
});

const initialize = async (user: PublicKey, program: any, keypair?: Keypair|undefined): Promise<PublicKey> => {
  const [pda, _] = PublicKey
    .findProgramAddressSync([
        anchor.utils.bytes.utf8.encode("RoboSwap"),
        user.toBuffer(),
      ],
      program.programId,
    )

    try {

    let tx = program.methods
      .initialize()
      .accounts({
        user: user,
        pda,
      })
      if (keypair) {
        tx.signers([keypair])
      }
      await tx.rpc();
    }
  catch (err) {
    logger.error(err)
    assert(false, err)
  }
  return pda
}

const airdrop = async (pubkey: PublicKey, provider: anchor.AnchorProvider) => {
  try {
    const signature = await provider.connection.requestAirdrop(
      pubkey, 10000000000
    )
  }
  catch (err) {
    assert(false, err)
  }
}

const createAccount = async (keypair: Keypair, provider: anchor.AnchorProvider) => {
  try {
    const instruction = anchor.web3.SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: keypair.publicKey,
      space: 0,
      lamports: 100000000,
      programId: anchor.web3.SystemProgram.programId,
    })
    let tx = new anchor.web3.Transaction().add(instruction)
    const latestBlockhash = await provider.connection.getLatestBlockhash()
    tx.recentBlockhash = latestBlockhash.blockhash
    tx.feePayer = provider.wallet.publicKey
    tx = await new anchor.Wallet(keypair).signTransaction(tx)
    const signature = await provider.sendAndConfirm(tx)
  }
  catch (err) {
    assert(false)
  }
  
}