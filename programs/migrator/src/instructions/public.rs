//! Instructions callable by anyone.

use crate::account_contexts::*;
use crate::bpf_loader_upgradeable::UpgradeableLoaderAccount;
use anchor_lang::prelude::*;
use vipers::unwrap_int;

/// Proposes a [Migration].
pub fn propose_migration(
    ctx: Context<ProposeMigration>,
    bump: u8,
    title: String,
    description: String,
) -> ProgramResult {
    let migrator = &mut ctx.accounts.migrator;
    let index = migrator.num_migrations;
    migrator.num_migrations = unwrap_int!(migrator.num_migrations.checked_add(1));

    let migration = &mut ctx.accounts.migration;
    migration.migrator = migrator.key();
    migration.index = index;
    migration.bump = bump;

    migration.buffer = ctx.accounts.buffer.key();
    migration.proposer = ctx.accounts.proposer.key();

    migration.created_at = Clock::get()?.unix_timestamp;
    migration.rejected_at = -1;
    migration.executed_at = -1;
    migration.executor = Pubkey::default();

    migration.title = title;
    migration.description = description;

    Ok(())
}

/// Reserves a new program ID to be administered by its migrator.
pub fn reserve_program_id(ctx: Context<ReserveProgramID>) -> ProgramResult {
    let program_address = ctx.accounts.program.key();

    let rent = Rent::get()?;
    let min_program_balance =
        1.max(rent.minimum_balance(UpgradeableLoaderAccount::program_len().unwrap()));

    // Allocate the program account to later be assigned to the [bpf_loader_upgradeable].
    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            ctx.accounts.payer.key,
            &program_address,
            min_program_balance,
            UpgradeableLoaderAccount::program_len().unwrap() as u64,
            &crate::ID,
        ),
        &[
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.payer.to_account_info(),
        ],
    )?;

    Ok(())
}
