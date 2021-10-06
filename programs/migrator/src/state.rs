//! State for the migrator program.

use anchor_lang::prelude::*;

/// Migrates programs.
#[account]
#[derive(Default)]
pub struct Migrator {
    /// Program ID of the program to deploy.
    pub program_id: Pubkey,
    /// Bump seed.
    pub bump: u8,

    /// Authority which can approve migrations.
    pub approver: Pubkey,
    /// The current [Migration] that is approved for anyone to deploy.
    /// Only one [Migration] may be approved at a time.
    pub pending_migration: Pubkey,
    /// If >0, this timestamp marks when the approval for the program
    /// deployment/upgrade expires.
    /// If <= 0, there is considered to be no approved migration.
    pub approval_expires_at: i64,

    /// Total number of migrations that have been proposed to this [Migrator].
    pub num_migrations: u64,
    /// Index of the latest migration to have taken place.
    pub latest_migration_index: u64,

    /// User-friendly name of the program.
    pub name: String,
    /// Description of the program.
    pub description: String,
}

#[account]
#[derive(Default)]
pub struct Migration {
    /// The [Pubkey] of the [Migrator].
    pub migrator: Pubkey,
    /// The unique index of the [Migration]. Must be non-zero.
    pub index: u64,
    /// Bump seed.
    pub bump: u8,

    /// The key of the buffer to migrate to.
    /// This must be set to the [Migrator].
    pub buffer: Pubkey,
    /// The [Pubkey] that proposed this [Migration].
    pub proposer: Pubkey,

    /// When the [Migration] was created.
    pub created_at: i64,
    /// If the [Migrator] rejected this [Migration], this is the timestamp when the migration was rejected.
    /// This also allows us to filter out spam.
    pub rejected_at: i64,
    /// Timestamp of when this migration was executed. -1 if never executed.
    pub executed_at: i64,
    /// The [Pubkey] that executed this [Migration].
    pub executor: Pubkey,

    /// Title describing the migration
    pub title: String,
    /// Description of the migration. It is recommended to use Markdown.
    pub description: String,
}
