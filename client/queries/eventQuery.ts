import { Connection, PublicKey } from "@solana/web3.js";
import { AccountQuery, PublicKeyCriterion } from "./queries";
import { Event } from "../accounts";

export class Events extends AccountQuery {
  public static eventQuery(connection: Connection) {
    return new Events(connection);
  }

  constructor(connection: Connection) {
    super(connection, Event, new Map([
        ["authority",  new PublicKeyCriterion(8)],
      ])
    );
  }

  filterByAuthority(authority: PublicKey): Events {
    this.filters.get("authority").setValue(authority);
    return this;
  }
}
