import * as anchor from "@coral-xyz/anchor";
import {
  createSubcategory,
  createClassification,
  createEventGroup,
} from "../util/test_util";
import assert from "assert";
import { getAnchorProvider } from "../../admin/util";
import { sportClassificationPda } from "../util/pda";

describe("Create Grouping Accounts", () => {
  it("Create Classification - Success", async () => {
    const program = anchor.workspace.ProtocolEvent;

    const code = "POLITICS";
    const name = "Politics";
    const classificationPk = await createClassification(program, code, name);

    const classification = await program.account.classification.fetch(
      classificationPk,
    );
    assert.equal(code, classification.code);
    assert.equal(name, classification.name);
    assert.equal(
      getAnchorProvider().publicKey.toBase58(),
      classification.payer.toBase58(),
    );
  });

  it("Create Subcategory - Success", async () => {
    const program = anchor.workspace.ProtocolEvent;

    const code = "SC";
    const name = "Code Collecting";
    const subcategoryPk = await createSubcategory(
      program,
      sportClassificationPda(),
      code,
      name,
    );

    const subcategory = await program.account.subcategory.fetch(subcategoryPk);
    assert.equal(code, subcategory.code);
    assert.equal(name, subcategory.name);
    assert.equal(0, subcategory.participantCount);
    assert.equal(
      getAnchorProvider().publicKey.toBase58(),
      subcategory.payer.toBase58(),
    );
  });

  it("Create Event Group - Success", async () => {
    const program = anchor.workspace.ProtocolEvent;

    const subcategoryPk = await createSubcategory(
      program,
      sportClassificationPda(),
      "MUSH",
      "Mushroom Stomping",
    );

    const code = "MUSHUK";
    const name = "Mushroom Stompers Association UK";
    const eventGroupPk = await createEventGroup(
      program,
      subcategoryPk,
      code,
      name,
    );

    const eventGroup = await program.account.eventGroup.fetch(eventGroupPk);
    assert.equal(subcategoryPk.toBase58(), eventGroup.subcategory.toBase58());
    assert.equal(code, eventGroup.code);
    assert.equal(name, eventGroup.name);
    assert.equal(
      getAnchorProvider().publicKey.toBase58(),
      eventGroup.payer.toBase58(),
    );
  });
});
