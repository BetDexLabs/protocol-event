import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";

export function findEventPda(code: string, program: Program): PublicKey {
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("event"), Buffer.from(code)],
    program.programId,
  );
  return pda;
}

export function findClassificationPda(
  code: string,
  program: Program,
): PublicKey {
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("classification"), Buffer.from(code)],
    program.programId,
  );
  return pda;
}

export function findSubcategoryPda(
  classification: PublicKey,
  code: string,
  program: Program,
): PublicKey {
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("subcategory"), classification.toBuffer(), Buffer.from(code)],
    program.programId,
  );
  return pda;
}

export function findEventGroupPda(
  subcategory: PublicKey,
  code: string,
  program: Program,
): PublicKey {
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("event_group"), subcategory.toBuffer(), Buffer.from(code)],
    program.programId,
  );
  return pda;
}

export function sportClassificationPda(): PublicKey {
  const program: anchor.Program = anchor.workspace.ProtocolEvent;
  return findClassificationPda("SPORT", program);
}

export function footballSubcategoryPda(): PublicKey {
  const program: anchor.Program = anchor.workspace.ProtocolEvent;
  return findSubcategoryPda(sportClassificationPda(), "FOOTBALL", program);
}

export function eplEventGroupPda(): PublicKey {
  const program: anchor.Program = anchor.workspace.ProtocolEvent;
  return findEventGroupPda(footballSubcategoryPda(), "EPL", program);
}

export function findParticipantPda(
  subcategory: PublicKey,
  id: number,
  program: Program,
): PublicKey {
  const [pda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("participant"),
      subcategory.toBuffer(),
      Buffer.from(id.toString()),
    ],
    program.programId,
  );
  return pda;
}
