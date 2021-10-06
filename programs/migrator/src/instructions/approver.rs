//! Instructions callable by the approver.

use crate::account_contexts::*;
use anchor_lang::prelude::*;
use solana_program::bpf_loader_upgradeable;
use solana_program::{
    loader_upgradeable_instruction::UpgradeableLoaderInstruction, system_program, sysvar,
};
use vipers::unwrap_int;

/// Creates a new [Migrator].
pub fn new_migrator(
    ctx: Context<NewMigrator>,
    bump: u8,
    name: String,
    description: String,
) -> ProgramResult {
    let migrator = &mut ctx.accounts.migrator;
    migrator.program_id = ctx.accounts.program.key();
    migrator.bump = bump;

    migrator.approver = ctx.accounts.approver.key();
    migrator.pending_migration = Pubkey::default();
    migrator.approval_expires_at = -1;

    migrator.num_migrations = 0;
    migrator.name = name;
    migrator.description = description;

    Ok(())
}

/// Deploys a program with a migration.
pub fn deploy_program(ctx: Context<DeployProgram>) -> ProgramResult {
    let migrator = &ctx.accounts.approved_migration.migrator;
    let seeds = gen_migrator_signer_seeds!(migrator);

    // assign the account to bpf_loader_upgradeable
    solana_program::program::invoke_signed(
        &solana_program::system_instruction::assign(
            &migrator.program_id,
            &bpf_loader_upgradeable::ID,
        ),
        &[
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts
                .bpf_loader_upgradeable_program
                .to_account_info(),
        ],
        &[&seeds[..]],
    )?;

    // deploy the migration
    // for the first deploy, we will use a max data len of 2x the buffer.
    let buffer_size: usize = ctx
        .accounts
        .approved_migration
        .buffer
        .to_account_info()
        .data_len();
    let max_data_len = unwrap_int!(buffer_size.checked_mul(2));

    let deploy_ix = solana_program::instruction::Instruction::new_with_bincode(
        bpf_loader_upgradeable::ID,
        &UpgradeableLoaderInstruction::DeployWithMaxDataLen { max_data_len },
        vec![
            AccountMeta::new(migrator.key(), true),
            AccountMeta::new(ctx.accounts.program.program_data.key(), false),
            AccountMeta::new(ctx.accounts.program.program.key(), false),
            AccountMeta::new(ctx.accounts.approved_migration.buffer.key(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(migrator.key(), true),
        ],
    );
    solana_program::program::invoke_signed(
        &deploy_ix,
        &[
            migrator.to_account_info(),
            ctx.accounts.program.program_data.to_account_info(),
            ctx.accounts.program.program.to_account_info(),
            ctx.accounts.approved_migration.buffer.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.clock.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            migrator.to_account_info(),
        ],
        &[&seeds[..]],
    )?;

    ctx.accounts.approved_migration.commit()?;
    Ok(())
}

/// Upgrades a program.
pub fn upgrade_program(ctx: Context<UpgradeProgram>) -> ProgramResult {
    let migrator = &ctx.accounts.approved_migration.migrator;

    // upgrade the program
    let seeds = gen_migrator_signer_seeds!(migrator);
    let upgrade_ix = solana_program::bpf_loader_upgradeable::upgrade(
        ctx.accounts.program.program.to_account_info().key,
        ctx.accounts.approved_migration.buffer.to_account_info().key,
        migrator.to_account_info().key,
        migrator.to_account_info().key,
    );
    solana_program::program::invoke_signed(
        &upgrade_ix,
        &[
            ctx.accounts.program.program_data.to_account_info(),
            ctx.accounts.program.program.to_account_info(),
            ctx.accounts.approved_migration.buffer.to_account_info(),
            migrator.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.clock.to_account_info(),
            migrator.to_account_info(),
        ],
        &[&seeds[..]],
    )?;

    ctx.accounts.approved_migration.commit()?;
    Ok(())
}

/// Approves a [Migration].
pub fn approve_migration(ctx: Context<ApproveMigration>, deadline: i64) -> ProgramResult {
    require!(
        deadline > Clock::get()?.unix_timestamp,
        ExpiryMustBeInFuture
    );

    // un-reject if the migration was rejected.
    let migration = &mut ctx.accounts.migration;
    if migration.rejected_at != -1 {
        migration.rejected_at = -1;
    }

    let migrator = &mut ctx.accounts.migrator;
    migrator.pending_migration = ctx.accounts.migration.key();
    migrator.approval_expires_at = deadline;
    Ok(())
}

/// Rejects the current [Migration].
pub fn reject_migration(ctx: Context<RejectMigration>) -> ProgramResult {
    let migration = &mut ctx.accounts.migration;
    migration.rejected_at = Clock::get()?.unix_timestamp;

    // cancel migration if it's the pending one
    let migrator = &mut ctx.accounts.migrator;
    if migrator.pending_migration.key() == migration.key() {
        migrator.pending_migration = Pubkey::default();
        migrator.approval_expires_at = -1;
    }

    Ok(())
}
