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
use vipers::validate::Validate;

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
        ctx.accounts.validate()?;
        instructions::approver::new_migrator(ctx, bump, name, description)
    }

    /// Deploys a program with a migration.
    pub fn deploy_program(ctx: Context<DeployProgram>) -> ProgramResult {
        ctx.accounts.validate()?;
        instructions::approver::deploy_program(ctx)
    }

    /// Upgrades a program.
    pub fn upgrade_program(ctx: Context<UpgradeProgram>) -> ProgramResult {
        ctx.accounts.validate()?;
        instructions::approver::upgrade_program(ctx)
    }

    /// Approves a [Migration].
    pub fn approve_migration(ctx: Context<ApproveMigration>, deadline: i64) -> ProgramResult {
        ctx.accounts.validate()?;
        instructions::approver::approve_migration(ctx, deadline)
    }
    /// Approves a [Migration].
    pub fn reject_migration(ctx: Context<RejectMigration>) -> ProgramResult {
        ctx.accounts.validate()?;
        instructions::approver::reject_migration(ctx)
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
        ctx.accounts.validate()?;
        instructions::public::propose_migration(ctx, bump, title, description)
    }

    /// Reserves a new program ID to be administered by its migrator.
    pub fn reserve_program_id(ctx: Context<ReserveProgramID>) -> ProgramResult {
        ctx.accounts.validate()?;
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
