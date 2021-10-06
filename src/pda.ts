import { utils } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";

import { PROGRAM_ID } from "./constants";

export const findMigratorKey = async (
  programID: PublicKey
): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [utils.bytes.utf8.encode("migrator"), programID.toBytes()],
    PROGRAM_ID
  );
};
