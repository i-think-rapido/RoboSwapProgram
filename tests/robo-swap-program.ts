import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { RoboSwapProgram } from "../target/types/robo_swap_program";
import _ from "lodash";

describe("robo-swap-program", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.RoboSwapProgram as Program<RoboSwapProgram>;

  it("Is initialized!", async () => {

    const [gamePDA, _bump] = PublicKey
      .findProgramAddressSync([
          anchor.utils.bytes.utf8.encode("RoboSwap"),
          provider.wallet.publicKey.toBuffer(),
        ],
        program.programId,
      )

    try {

    const tx = await program.methods
      .initialize(provider.wallet.publicKey)
      .accounts({
        user: provider.wallet.publicKey,
        game: gamePDA,
      })
      .rpc();
    }
    catch (err) {
      console.error(err)
    }

    const robots = (await program.account.game.fetch(gamePDA)).robots;
    _.range(26).map(d => {
      const robot = robots[d]
      expect(robot.wallet.toBase58()).to.equal(provider.wallet.publicKey.toBase58())
      expect(robot.owner.toBase58()).to.equal(provider.wallet.publicKey.toBase58())
      expect(robot.idx).to.equal(d)
      expect(robot.ownerIdx).to.equal(d)
      expect(robot.swaps).to.equal(0)
    })
  });
});
