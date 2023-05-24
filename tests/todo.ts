import * as anchor from "@coral-xyz/anchor";
import { readFileSync } from 'fs';
import { Todo } from "../target/types/todo";

export const TODO_PROGRAM_PUBKEY = '5kZtVwH69P8uUH6fZ1Dd4Fh55H4254vNnigWZ8VAZirp';

describe("todo", () => {
  let ap = anchor.AnchorProvider.env();
  anchor.setProvider(ap);

  const idl = JSON.parse(readFileSync("./target/idl/todo.json", "utf8"))
  const program = new anchor.Program<Todo>(idl, TODO_PROGRAM_PUBKEY);

  const programProvider = program.provider as anchor.AnchorProvider;

  it("Is initialized!", async () => {
    const userProfile = anchor.web3.Keypair.generate();
    const payer = programProvider.wallet;

    const tx = await program.methods.initialize()
      .accounts({
        authority: payer.publicKey,
        userProfile: userProfile.publicKey,
      })
      .signers([userProfile])
      .rpc();

    console.log("Your transaction signature", tx);

    let state = await program.account.userProfile.fetch(userProfile.publicKey);
    console.log("state", state);
  });
});
