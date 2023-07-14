import { Connection, PublicKey } from "@solana/web3.js";
import { GET_MULTIPLE_ACCOUNTS_LIMIT, PublicKeyCriterion, toFilters } from "./queries";
import { PROGRAM_ID } from "../programId";
import { Event } from "../accounts";

export class Events {
  public static eventQuery(connection: Connection) {
    return new Events(connection);
  }

  private readonly connection: Connection;

  private authority: PublicKeyCriterion = new PublicKeyCriterion(8);

  constructor(connection: Connection) {
    this.connection = connection;
  }

  filterByAuthority(authority: PublicKey): Events {
    this.authority.setValue(authority);
    return this;
  }

  /**
   *
   * @returns: list of all fetched event publicKeys
   */
  async fetchPublicKeys() {
    try {
      const accounts = await this.connection.getProgramAccounts(
        PROGRAM_ID,
        {
          dataSlice: { offset: 0, length: 0 }, // fetch without any data.
          filters: toFilters("event", this.authority)
        }
      );
      return accounts.map((account) => account.pubkey);
    } catch (e) {
      console.error(e);
      throw e;
    }
  }

  /**
   *
   * @returns fetched event accounts mapped to their publicKey
   */
  async fetch() {
    const publicKeys = await this.fetchPublicKeys();

    let events: Event[] = [];

    try {
      if (publicKeys.length <= GET_MULTIPLE_ACCOUNTS_LIMIT) {
        events = await Event.fetchMultiple(this.connection, publicKeys);
      } else {
        for (let i = 0; i < publicKeys.length; i += GET_MULTIPLE_ACCOUNTS_LIMIT) {
          const eventsChunk = await Event.fetchMultiple(this.connection, publicKeys.slice(i, i + GET_MULTIPLE_ACCOUNTS_LIMIT));
          events = events.concat(eventsChunk);
        }
      }

      return publicKeys
        .map((publicKey, i) => {
          return { publicKey: publicKey, account: events[i] };
        })
        .filter((o) => o.account);
    } catch (e) {
      console.error(e);
      throw e;
    }
  }
}
