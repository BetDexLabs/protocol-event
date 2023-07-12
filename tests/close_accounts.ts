import {
  createCategory,
  createEvent,
  createEventGroup,
  createIndividualParticipant,
  createTeamParticipant, createWalletWithBalance
} from "./util/test_util";
import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { ProtocolEvent } from "../target/types/protocol_event";
import assert from "assert";
import console from "console";

describe("Close Accounts", () => {
  it("Success", async () => {
    const console = require("console");

    const program = anchor.workspace
      .ProtocolEvent as Program<ProtocolEvent>;


    const payer = await createWalletWithBalance();
    const startingBalance = await program.provider.connection.getBalance(payer.publicKey);

    const categoryPk = await createCategory(program, "CLOSE", "To Close", payer);
    const eventGroupPk = await createEventGroup(program, categoryPk, "CLOSE", "To Close", payer);
    const individualPk = await createIndividualParticipant(program, categoryPk, "CLOSE", "To Close", payer);
    const teamPk = await createTeamParticipant(program, categoryPk, "CLOSE", "To Close", payer);
    const eventPk = await createEvent({
        code: "CLOSE",
        name: "TO CLOSE",
        participants: [],
        actualEndTimestamp: null,
        actualStartTimestamp: null,
        expectedStartTimestamp: new BN(1689169672)
      },
      categoryPk,
      eventGroupPk,
      payer
    );

    await program.methods.closeCategory()
      .accounts({
        category: categoryPk,
        authority: payer.publicKey,
        payer: payer.publicKey
      })
      .signers([payer])
      .rpc();

    await program.methods.closeEventGroup()
      .accounts({
        eventGroup: eventGroupPk,
        authority: payer.publicKey,
        payer: payer.publicKey
      })
      .signers([payer])
      .rpc();

    console.log((await program.account.participant.fetch(individualPk)).payer.toBase58())
    await program.methods.closeParticipant()
      .accounts({
        participant: individualPk,
        authority: payer.publicKey,
        payer: payer.publicKey
      })
      .signers([payer])
      .rpc()
      .catch((e) => console.log(e));
    console.log(1)

    console.log((await program.account.participant.fetch(teamPk)).payer.toBase58())
    await program.methods.closeParticipant()
      .accounts({
        participant: teamPk,
        authority: payer.publicKey,
        payer: payer.publicKey
      })
      .signers([payer])
      .rpc()
      .catch((e) => console.log(e));
    console.log(2)

    const closingBalance = await program.provider.connection.getBalance(payer.publicKey);
    assert.equal(startingBalance, closingBalance);
  });
});