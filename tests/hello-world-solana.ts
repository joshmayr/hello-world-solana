import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SystemProgram } from "@solana/web3.js";
import { HelloWorldSolana } from "../target/types/hello_world_solana";
import * as assert from "assert";

describe("hello-world-solana", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.HelloWorldSolana as Program<HelloWorldSolana>;

  const baseAccount = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    const tx = await program.rpc.initialize({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: anchor.getProvider().publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });
  });

  it("Get message count", async () => {
    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    assert.ok(account.totalMessages.eq(new anchor.BN(0)));
  });

  it("Send a message", async () => {
    let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    assert.ok(account.totalMessages.eq(new anchor.BN(0)));
    const tx = await program.rpc.sendMessage("Hello, World!", {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: anchor.getProvider().publicKey,
      },
    });

    account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    assert.ok(account.totalMessages.eq(new anchor.BN(1)));
    assert.equal(account.messageList[0].message, "Hello, World!");
  });
});
