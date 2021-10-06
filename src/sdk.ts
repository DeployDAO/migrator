import { Program, Provider as AnchorProvider } from "@project-serum/anchor";
import type { Provider } from "@saberhq/solana-contrib";
import { SignerWallet, SolanaProvider } from "@saberhq/solana-contrib";
import type { Signer } from "@solana/web3.js";

import { MigratorJSON } from ".";
import { PROGRAM_ID } from "./constants";
import type { MigratorProgram } from "./types";

export class MigratorSDK {
  constructor(
    public readonly provider: Provider,
    public readonly program: MigratorProgram
  ) {}

  withSigner(signer: Signer): MigratorSDK {
    return MigratorSDK.load({
      provider: new SolanaProvider(
        this.provider.connection,
        this.provider.broadcaster,
        new SignerWallet(signer),
        this.provider.opts
      ),
    });
  }

  /**
   * Loads the SDK.
   * @returns {MigratorSDK}
   */
  public static load({
    provider,
  }: {
    // Provider
    provider: Provider;
  }): MigratorSDK {
    const anchorProvider = new AnchorProvider(
      provider.connection,
      provider.wallet,
      provider.opts
    );
    return new MigratorSDK(
      provider,
      new Program(
        MigratorJSON,
        PROGRAM_ID,
        anchorProvider
      ) as unknown as MigratorProgram
    );
  }
}
