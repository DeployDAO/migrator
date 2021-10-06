import type { AnchorTypes } from "@saberhq/anchor-contrib";

import type { MigratorIDL } from "./idls/migrator";

export * from "./idls/migrator";

export type MigratorTypes = AnchorTypes<
  MigratorIDL,
  {
    migrator: MigratorData;
    migration: MigrationData;
  }
>;

type Accounts = MigratorTypes["Accounts"];
export type MigratorData = Accounts["Migrator"];
export type MigrationData = Accounts["Migration"];

export type MigratorError = MigratorTypes["Error"];
export type MigratorEvents = MigratorTypes["Events"];
export type MigratorProgram = MigratorTypes["Program"];
