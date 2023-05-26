import * as anchor from "@coral-xyz/anchor";
import { readFileSync } from 'fs';
import { expect } from "chai";
import { Todo } from "../target/types/todo";

export const TODO_PROGRAM_PUBKEY = '5kZtVwH69P8uUH6fZ1Dd4Fh55H4254vNnigWZ8VAZirp';

describe("todo", () => {
  let ap = anchor.AnchorProvider.env();
  anchor.setProvider(ap);

  const idl = JSON.parse(readFileSync("./target/idl/todo.json", "utf8"))
  const program = new anchor.Program<Todo>(idl, TODO_PROGRAM_PUBKEY);

  const programProvider = program.provider as anchor.AnchorProvider;
  const payer = programProvider.wallet;
  const userProfile = anchor.web3.Keypair.generate();

  it("Create User", async () => {
    const [name, username, pwd] = ["john", "superJohn", "pass"];

    await program.methods.createUser(name, username, pwd)
      .accounts({
        authority: payer.publicKey,
        userProfile: userProfile.publicKey,
      })
      .signers([userProfile])
      .rpc();

    const user = await program.account.userProfile.fetch(userProfile.publicKey);

    expect(user.authority).to.eql(payer.publicKey);
    expect(user.name).to.eql(name);
    expect(user.username).to.eql(username);
    expect(user.password).to.eql(pwd);
  });

  it("Create List", async () => {
    const listAccount = anchor.web3.Keypair.generate();
    const [title, description] = ["test title", "test description"];

    await program.methods.createList(title, description)
      .accounts({
        authority: payer.publicKey,
        userProfile: userProfile.publicKey,
        listAccount: listAccount.publicKey,
      })
      .signers([listAccount])
      .rpc();

    let list = await program.account.listAccount.fetch(listAccount.publicKey);

    expect(list.title).to.eql(title);
    expect(list.description).to.eql(description);
    expect(list.status).to.eql({ active: {} });
    expect(list.todos).to.eql([]);
  });
});
