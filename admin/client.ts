import { getAnchorProvider } from "./util";
import * as anchor from "@coral-xyz/anchor";
import { addEventParticipants, createEvent } from "./createEvent";
import { createCategory } from "./category";
import { createEventGroup } from "./eventGroup";
import { createParticipant } from "./participant";

if (process.argv.length < 3) {
  printUsageAndExit();
}

const provider = getAnchorProvider();
anchor.setProvider(provider);

const script = process.argv[2];

switch (script) {
  case "createEvent":
    createEvent();
    break;
  case "createCategory":
    createCategory();
    break;
  case "createEventGroup":
    createEventGroup();
    break;
  case "createParticipant":
    createParticipant();
    break;
  case "addEventParticipants":
    addEventParticipants();
    break;
  default:
    printUsageAndExit();
    break;
}

function printUsageAndExit() {
  console.log("Usage: yarn ts-node <command> <args> ...");
  process.exit(1);
}
