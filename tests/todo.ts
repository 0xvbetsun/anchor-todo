import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { IDL } from "../target/types/todo";

const TODO_PROGRAM_PUBKEY = new anchor.web3.PublicKey(
  "8uvpDc9tZxwYspeqX37HtDTBQPzJZU7Gp4GaKU8qz4Us"
);

const expectUser = ({ user, authority, name, username, password, listIdx }) => {
  expect(user.authority).to.eql(authority);
  expect(user.name).to.eql(name);
  expect(user.username).to.eql(username);
  expect(user.password).to.eql(password);
  expect(user.listIdx).to.eql(listIdx);
};

const expectList = ({ list, id, title, description }) => {
  expect(list.id).to.eql(id);
  expect(list.title).to.eql(title);
  expect(list.description).to.eql(description);
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
      listIdx: 1,
    });
  });

  describe("List", () => {
    const listId = 1;
    const [listPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("LIST_STATE"),
        userProfile.publicKey.toBuffer(),
        Buffer.from([listId]),
      ],
      TODO_PROGRAM_PUBKEY
    );
    console.log("list account: ", listPDA.toString());

    it("Create List", async () => {
      const [title, description] = ["test title", "test description"];

      await program.methods
        .createList(title, description)
        .accounts({
          authority: payer.publicKey,
          userProfile: userProfile.publicKey,
          listAccount: listPDA,
        })
        .rpc();

      const [list, usersLists] = await Promise.all([
        program.account.listAccount.fetch(listPDA),
        program.account.listAccount.all([
          {
            memcmp: {
              offset: 8, // Discriminator.
              bytes: userProfile.publicKey.toBase58(),
            },
          },
        ]),
      ]);

      expectList({
        list,
        id: listId,
        title,
        description,
      });

      expect(usersLists.length).eql(1);
    });

    it("Update List", async () => {
      const [newTitle, newDescription] = [
        "new test title",
        "new test description",
      ];

      await program.methods
        .updateList(listId, newTitle, newDescription)
        .accounts({
          authority: payer.publicKey,
          userProfile: userProfile.publicKey,
          listAccount: listPDA,
        })
        .rpc();

      let list = await program.account.listAccount.fetch(listPDA);

      expectList({
        list,
        id: listId,
        title: newTitle,
        description: newDescription,
      });
    });

    it("Remove List", async () => {
      await program.methods
        .removeList()
        .accounts({
          authority: payer.publicKey,
          userProfile: userProfile.publicKey,
          listAccount: listPDA,
        })
        .rpc();

      const [list, usersLists] = await Promise.all([
        program.account.listAccount.fetchNullable(listPDA),
        program.account.listAccount.all([
          {
            memcmp: {
              offset: 8, // Discriminator.
              bytes: userProfile.publicKey.toBase58(),
            },
          },
        ]),
      ]);

      expect(list).to.eql(null);
      expect(usersLists.length).eql(0);
    });
  });
});
