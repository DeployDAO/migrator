import type { PublicKey } from "@solana/web3.js";

import type { MigratorSDK } from "./sdk";

export class MigratorWrapper {
  constructor(
    public readonly sdk: MigratorSDK,
    public readonly migratorKey: PublicKey
  ) {}
}
