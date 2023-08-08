import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface SubcategoryFields {
  authority: PublicKey
  classification: PublicKey
  code: string
  name: string
  participantCount: number
  payer: PublicKey
}

export interface SubcategoryJSON {
  authority: string
  classification: string
  code: string
  name: string
  participantCount: number
  payer: string
}

export class Subcategory {
  readonly authority: PublicKey
  readonly classification: PublicKey
  readonly code: string
  readonly name: string
  readonly participantCount: number
  readonly payer: PublicKey

  static readonly discriminator = Buffer.from([
    78, 24, 222, 4, 37, 250, 237, 162,
  ])

  static readonly layout = borsh.struct([
    borsh.publicKey("authority"),
    borsh.publicKey("classification"),
    borsh.str("code"),
    borsh.str("name"),
    borsh.u16("participantCount"),
    borsh.publicKey("payer"),
  ])

  constructor(fields: SubcategoryFields) {
    this.authority = fields.authority
    this.classification = fields.classification
    this.code = fields.code
    this.name = fields.name
    this.participantCount = fields.participantCount
    this.payer = fields.payer
  }

  static async fetch(
    c: Connection,
    address: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): Promise<Subcategory | null> {
    const info = await c.getAccountInfo(address)

    if (info === null) {
      return null
    }
    if (!info.owner.equals(programId)) {
      throw new Error("account doesn't belong to this program")
    }

    return this.decode(info.data)
  }

  static async fetchMultiple(
    c: Connection,
    addresses: PublicKey[],
    programId: PublicKey = PROGRAM_ID
  ): Promise<Array<Subcategory | null>> {
    const infos = await c.getMultipleAccountsInfo(addresses)

    return infos.map((info) => {
      if (info === null) {
        return null
      }
      if (!info.owner.equals(programId)) {
        throw new Error("account doesn't belong to this program")
      }

      return this.decode(info.data)
    })
  }

  static decode(data: Buffer): Subcategory {
    if (!data.slice(0, 8).equals(Subcategory.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = Subcategory.layout.decode(data.slice(8))

    return new Subcategory({
      authority: dec.authority,
      classification: dec.classification,
      code: dec.code,
      name: dec.name,
      participantCount: dec.participantCount,
      payer: dec.payer,
    })
  }

  toJSON(): SubcategoryJSON {
    return {
      authority: this.authority.toString(),
      classification: this.classification.toString(),
      code: this.code,
      name: this.name,
      participantCount: this.participantCount,
      payer: this.payer.toString(),
    }
  }

  static fromJSON(obj: SubcategoryJSON): Subcategory {
    return new Subcategory({
      authority: new PublicKey(obj.authority),
      classification: new PublicKey(obj.classification),
      code: obj.code,
      name: obj.name,
      participantCount: obj.participantCount,
      payer: new PublicKey(obj.payer),
    })
  }
}
