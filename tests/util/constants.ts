import { BN } from "@coral-xyz/anchor";

// TODO port these to monaco client
export type CreateEventInfo = {
  slug: string;
  name: string;
  participants: number[];
  expectedStartTimestamp: BN;
  actualStartTimestamp: BN | null;
  actualEndTimestamp: BN | null;
};

export enum ParticipantType {
  Individual,
  Team,
}
