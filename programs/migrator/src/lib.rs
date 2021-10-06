//! Program for managing and outsourcing program deploys and upgrades.

#[macro_use]
mod macros;

pub mod account_contexts;
mod account_validators;
pub mod bpf_loader_upgradeable;
pub mod instructions;
mod migrate;
pub mod state;

use account_contexts::*;
use anchor_lang::prelude::*;

declare_id!("M1G1VdgdfvjMCdUhVtzaejnutPmLknEiraq2F59YGxr");

/// The [migrator] program.
#[program]
pub mod migrator {
    use super::*;

    //////////////////////////////////////////
    // Approver instructions
    //////////////////////////////////////////

    /// Creates a new [Migrator].
    pub fn new_migrator(
        ctx: Context<NewMigrator>,
        bump: u8,
        name: String,
        description: String,
    ) -> ProgramResult {
        instructions::approver::new_migrator(ctx, bump, name, description)
    }

    /// Deploys a program with a migration.
    pub fn deploy_program(ctx: Context<DeployProgram>) -> ProgramResult {
        instructions::approver::deploy_program(ctx)
    }

    /// Upgrades a program.
    pub fn upgrade_program(ctx: Context<UpgradeProgram>) -> ProgramResult {
        instructions::approver::upgrade_program(ctx)
    }

    /// Approves a [Migration].
    pub fn approve_migration(ctx: Context<ApproveMigration>, deadline: i64) -> ProgramResult {
        instructions::approver::approve_migration(ctx, deadline)
    }

    //////////////////////////////////////////
    // Public instructions
    //////////////////////////////////////////

    /// Proposes a [Migration].
    pub fn propose_migration(
        ctx: Context<ProposeMigration>,
        bump: u8,
        title: String,
        description: String,
    ) -> ProgramResult {
        instructions::public::propose_migration(ctx, bump, title, description)
    }

    /// Reserves a new program ID to be administered by its migrator.
    pub fn reserve_program_id(ctx: Context<ReserveProgramID>) -> ProgramResult {
        instructions::public::reserve_program_id(ctx)
    }
}

#[error]
pub enum ErrorCode {
    #[msg("Could not deserialize UpgradeableLoaderState.")]
    ParseError,
    #[msg("Must be signer of an uninitialized program.")]
    ProgramIdNotSigner,
    #[msg("Buffer authority mismatch.")]
    BufferAuthorityMismatch,
    #[msg("No approved migration.")]
    NoApprovedMigration,
    #[msg("Migration approval window expired.")]
    MigrationWindowExpired,
    #[msg("Insufficient lamports remaining for rent exemption.")]
    InsufficientLamports,
    #[msg("Migration rejected.")]
    MigrationRejected,
    #[msg("Migration already executed.")]
    MigrationAlreadyExecuted,
    #[msg("Migration expiry time must be in the future.")]
    ExpiryMustBeInFuture,
}
