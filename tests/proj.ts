import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Proj } from "../target/types/proj";
import { Keypair } from "@solana/web3.js";

describe("proj", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Proj as Program<Proj>;

  const counterAcc = new Keypair();

  it("Is initialized!", async () => {

    const tx = await program.methods
      .initialize()
      .accounts({
        counter: counterAcc.publicKey,
      })
      .signers([counterAcc])
      .rpc({skipPreflight: true});
      
    
      const AccountData = await program.account.counter.fetch(
        counterAcc.publicKey
      );

      console.log("Your transaction signature", tx);
      console.log(`Count: ${AccountData.count}`);
  });

  it("Is initialized!", async () => {

    const tx = await program.methods.increment().accounts({counter: counterAcc.publicKey}).rpc();
    const accData = await program.account.counter.fetch(counterAcc.publicKey);

    console.log("Your transaction signature", tx);
    console.log(`Count: ${accData.count}`);
  });
});
