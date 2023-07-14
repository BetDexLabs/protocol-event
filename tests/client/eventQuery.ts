import * as anchor from "@coral-xyz/anchor";
import { createEvent, createWalletWithBalance } from "../util/test_util";
import { CreateEventInfo } from "../util/constants";
import { eplEventGroupPda, footballCategoryPda } from "../util/pda";
import { Events } from "../../client/queries";
import assert from "assert";

describe("Test Event Query", () => {
  it("Fetch event using query", async () => {
    const program = anchor.workspace.ProtocolEvent;

    const firstAuthority = await createWalletWithBalance();
    const secondAuthority = await createWalletWithBalance();

    const name1 = "TEST NAME 1";
    const code1 = "test-name-1";
    const eventAuthority1Pk = await createEvent(
      {
        code: code1,
        name: name1,
        participants: [],
        expectedStartTimestamp: new anchor.BN(1924200000),
        actualStartTimestamp: null,
        actualEndTimestamp: null,
      } as CreateEventInfo,
      footballCategoryPda(),
      eplEventGroupPda(),
      firstAuthority,
    );

    const name2 = "TEST NAME 2";
    const code2 = "test-name-2";
    const eventAuthority2Pk = await createEvent(
      {
        code: code2,
        name: name2,
        participants: [],
        expectedStartTimestamp: new anchor.BN(1924200000),
        actualStartTimestamp: null,
        actualEndTimestamp: null,
      } as CreateEventInfo,
      footballCategoryPda(),
      eplEventGroupPda(),
      secondAuthority,
    );

    // fetch all events

    const eventsUnfiltered = await Events.eventQuery(
      program.provider.connection,
    ).fetch();

    const unfilteredPks = eventsUnfiltered.map((event) =>
      event.publicKey.toBase58(),
    );
    assert.deepEqual(
      [eventAuthority2Pk.toBase58(), eventAuthority1Pk.toBase58()],
      unfilteredPks,
    );

    const unfilteredAuthorities = eventsUnfiltered.map((event) =>
      event.account.authority.toBase58(),
    );
    assert.deepEqual(
      [
        secondAuthority.publicKey.toBase58(),
        firstAuthority.publicKey.toBase58(),
      ],
      unfilteredAuthorities,
    );

    // fetch events filtered by authority 1

    const eventsFilteredAuth1 = await Events.eventQuery(
      program.provider.connection,
    )
      .filterByAuthority(firstAuthority.publicKey)
      .fetch();

    const eventPkAuth1 = eventsFilteredAuth1.map((event) =>
      event.publicKey.toBase58(),
    );
    assert.deepEqual([eventAuthority1Pk.toBase58()], eventPkAuth1);

    const auth1Pk = eventsFilteredAuth1.map((event) =>
      event.account.authority.toBase58(),
    );
    assert.deepEqual([firstAuthority.publicKey.toBase58()], auth1Pk);
  });
});
