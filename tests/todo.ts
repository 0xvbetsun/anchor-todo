import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { IDL } from "../target/types/todo";

export const TODO_PROGRAM_PUBKEY =
  "5kZtVwH69P8uUH6fZ1Dd4Fh55H4254vNnigWZ8VAZirp";

const expectUser = ({ user, authority, name, username, password }) => {
  expect(user.authority).to.eql(authority);
  expect(user.name).to.eql(name);
  expect(user.username).to.eql(username);
  expect(user.password).to.eql(password);
};

const expectList = ({ list, title, description, todos }) => {
  expect(list.title).to.eql(title);
  expect(list.description).to.eql(description);
  expect(list.todos).to.eql(todos);
};

describe("todo", () => {
  let ap = anchor.AnchorProvider.env();
  anchor.setProvider(ap);

  const program = new anchor.Program(IDL, TODO_PROGRAM_PUBKEY);

  const programProvider = program.provider as anchor.AnchorProvider;
  const payer = programProvider.wallet;
  const userProfile = anchor.web3.Keypair.generate();
  console.log("user profile: ", userProfile.publicKey.toString());

  it("Create User", async () => {
    const [name, username, password] = ["john", "superJohn", "pass"];

    await program.methods
      .createUser(name, username, password)
      .accounts({
        authority: payer.publicKey,
        userProfile: userProfile.publicKey,
      })
      .signers([userProfile])
      .rpc();

    const user = await program.account.userProfile.fetch(userProfile.publicKey);

    expectUser({
      user,
      name,
      username,
      password,
      authority: payer.publicKey,
    });
  });

  describe("List", () => {
    // const listAccount = anchor.web3.Keypair.generate();
    // console.log("list account: ", listAccount.publicKey.toString());

    // it("Create List", async () => {
    //   const [title, description] = ["test title", "test description"];

    //   await program.methods
    //     .createList(title, description)
    //     .accounts({
    //       authority: payer.publicKey,
    //       userProfile: userProfile.publicKey,
    //       listAccount: listAccount.publicKey,
    //     })
    //     .signers([listAccount])
    //     .rpc();

    //   let [list, user] = await Promise.all([
    //     program.account.listAccount.fetch(listAccount.publicKey),
    //     program.account.userProfile.fetch(userProfile.publicKey),
    //   ]);

    //   expectList({
    //     list,
    //     title,
    //     description,
    //     todos: [],
    //   });

    //   expect(user.lists.map((l) => l.toString())).to.includes(
    //     listAccount.publicKey.toString()
    //   );
    // });

    // it("Update List", async () => {
    //   const [newTitle, newDescription] = [
    //     "new test title",
    //     "new test description",
    //   ];

    //   await program.methods
    //     .updateList(newTitle, newDescription)
    //     .accounts({
    //       authority: payer.publicKey,
    //       userProfile: userProfile.publicKey,
    //       listAccount: listAccount.publicKey,
    //     })
    //     .rpc();

    //   let list = await program.account.listAccount.fetch(listAccount.publicKey);

    //   expectList({
    //     list,
    //     title: newTitle,
    //     description: newDescription,
    //     todos: [],
    //   });
    // });

    // it("Remove List", async () => {
    //   await program.methods
    //     .removeList()
    //     .accounts({
    //       authority: payer.publicKey,
    //       userProfile: userProfile.publicKey,
    //       listAccount: listAccount.publicKey,
    //     })
    //     .rpc();

    //   const [list, user] = await Promise.all([
    //     program.account.listAccount.fetchNullable(listAccount.publicKey),
    //     program.account.userProfile.fetch(userProfile.publicKey),
    //   ]);

    //   expect(list).to.eql(null);
    //   expect(user.lists).to.eql([]);
    // });
  });
});
