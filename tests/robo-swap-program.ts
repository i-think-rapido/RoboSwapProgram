import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { RoboSwapProgram } from "../target/types/robo_swap_program";

describe("robo-swap-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RoboSwapProgram as Program<RoboSwapProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
