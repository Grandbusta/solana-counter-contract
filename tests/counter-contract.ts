import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterContract } from "../target/types/counter_contract";
import { expect } from "chai";

describe("counter-contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CounterContract as Program<CounterContract>;

  let counter: anchor.web3.Keypair;

  it("Is initialized!", async () => {
    // Add your test here.
    counter = anchor.web3.Keypair.generate();
    const tx = await program.methods.initialize().accounts({
      counter: counter.publicKey,
      payer: provider.wallet.publicKey,
    }).signers([counter]).rpc();

    const counterData = await program.account.counter.fetch(counter.publicKey);
    expect(counterData.count).to.equal(0);
  });

  it("Increments counter", async () => {
    const tx = await program.methods.increment().accounts({
      counter: counter.publicKey,
      payer: provider.wallet.publicKey,
    }).rpc();
    const counterData = await program.account.counter.fetch(counter.publicKey);
    expect(counterData.count).to.equal(1);
  });

  it("Decrements counter", async () => {
    const tx = await program.methods.decrement().accounts({
      counter: counter.publicKey,
      payer: provider.wallet.publicKey,
    }).rpc();
    const counterData = await program.account.counter.fetch(counter.publicKey);
    expect(counterData.count).to.equal(0);
  });
});
