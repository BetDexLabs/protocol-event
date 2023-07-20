import { PublicKey, SystemProgram } from "@solana/web3.js";
import { findEventPda } from "./pda";
import { getProgram, sendTransaction } from "./util";
import {
  createEvent as clientEventCreate,
  CreateEventAccounts,
  CreateEventArgs,
  CreateEventInfoFields,
  addEventParticipants as clientAddEventParticipants,
  AddEventParticipantsArgs,
  AddEventParticipantsAccounts,
} from "../client";
import { BN } from "@coral-xyz/anchor";

export async function createEvent() {
  if (process.argv.length < 7) {
    console.log(
      "Usage: yarn run createEvent <CODE> <NAME> <CATEGORY_PK> <EVENT_GROUP_PK> <EXPECTED_START_TIMESTAMP>",
    );
    process.exit(1);
  }

  const code = process.argv[3];
  const name = process.argv[4];
  const categoryPk = new PublicKey(process.argv[5]);
  const eventGroupPk = new PublicKey(process.argv[6]);
  const startTime = Number.parseInt(process.argv[7]);

  const program = await getProgram();

  const eventPk = await findEventPda(code, program);

  const createEventArgs = {
    eventInfo: {
      code: code,
      name: name,
      participants: [],
      expectedStartTimestamp: new BN(startTime),
      actualStartTimestamp: null,
      actualEndTimestamp: null,
    } as CreateEventInfoFields,
  } as CreateEventArgs;

  const createEventAccs = {
    event: eventPk,
    eventGroup: eventGroupPk,
    category: categoryPk,
    authority: program.provider.publicKey,
    systemProgram: SystemProgram.programId,
  } as CreateEventAccounts;

  const ix = await clientEventCreate(createEventArgs, createEventAccs);

  await sendTransaction([ix]);

  console.log(`{"eventPk": "${eventPk.toBase58()}"}`);
}

export async function addEventParticipants() {
  if (process.argv.length < 4) {
    console.log(
      "Usage: yarn run addEventParticipants <EVENT_PK> [<PARTICIPANT_IDS>]...",
    );
    process.exit(1);
  }

  const eventPk = new PublicKey(process.argv[3]);
  const participants = process.argv[4];

  const program = await getProgram();
  const event = await program.account.event.fetch(eventPk);

  console.log(participants);
  console.log(
    participants
      .slice(1, -1)
      .split(",")
      .map((id) => parseInt(id)),
  );

  const addEventParticipantsArgs = {
    code: event.code,
    participantsToAdd: participants
      .slice(1, -1)
      .split(",")
      .map((id) => parseInt(id)),
  } as AddEventParticipantsArgs;

  const addEventParticipantsAccounts = {
    event: eventPk,
    category: event.category,
    authority: program.provider.publicKey,
    systemProgram: SystemProgram.programId,
  } as AddEventParticipantsAccounts;

  const ix = clientAddEventParticipants(
    addEventParticipantsArgs,
    addEventParticipantsAccounts,
  );

  await sendTransaction([ix]);

  console.log(`{"eventPk": "${eventPk.toBase58()}"}`);
}
