import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorFunctionTutorial } from "../target/types/anchor_function_tutorial";

describe("anchor-function-tutorial", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorFunctionTutorial as Program<AnchorFunctionTutorial>;

  it("Call boaty mcboatface!", async () => {
    // Add your test here.
    const tx = await program.methods.boatyMcBoatface().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should add", async () => {
    const tx = await program.methods.add(new anchor.BN(1), new anchor.BN(2)).rpc();
    console.log("Your transaction signature", tx);
  });
  
  it("Should sub", async () => {
    const tx = await program.methods.sub(
    new anchor.BN(10),
    new anchor.BN(3)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should mul", async () => {
    const tx = await program.methods.mul(
    new anchor.BN(10),
    new anchor.BN(3)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Should div", async () => {
    const tx = await program.methods.div(
    new anchor.BN(10),
    new anchor.BN(3)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Func A", async () => {
    const tx = await program.methods.functionA().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Func B", async () => {
    const tx = await program.methods.functionB(
    new anchor.BN(10)).rpc();
    console.log("Your transaction signature", tx);
  });
});
