//! Accounts structs.

use crate::{
    bpf_loader_upgradeable::{BPFLoaderUpgradeable, UpgradeableLoaderAccount},
    state::*,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(bump: u8, name: String, description: String)]
pub struct NewMigrator<'info> {
    /// [Migrator].
    #[account(
        init,
        seeds = [
            b"migrator".as_ref(),
            program.key().to_bytes().as_ref()
        ],
        bump = bump,
        payer = payer,
        space = std::mem::size_of::<Migrator>() + name.as_bytes().len() + description.as_bytes().len()
    )]
    pub migrator: Account<'info, Migrator>,

    /// Account which will approve migrations.
    pub approver: UncheckedAccount<'info>,

    /// Program ID.
    pub program: UncheckedAccount<'info>,

    /// Address where the program data will be stored.
    pub program_data: UncheckedAccount<'info>,

    /// Payer of transactions.
    pub payer: Signer<'info>,

    /// The [System] program.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeployProgram<'info> {
    /// The approved [Migration] and its [Migrator].
    pub approved_migration: ApprovedMigration<'info>,

    /// The program which has not yet been deployed.
    pub program: UndeployedProgram<'info>,

    /// The [Rent] sysvar.
    pub rent: Sysvar<'info, Rent>,
    /// The [Clock] sysvar.
    pub clock: Sysvar<'info, Clock>,
    /// The [System] program.
    pub system_program: Program<'info, System>,
    /// The [bpf_loader_upgradeable] program.
    pub bpf_loader_upgradeable_program: Program<'info, BPFLoaderUpgradeable>,
}

#[derive(Accounts)]
pub struct UpgradeProgram<'info> {
    /// The approved [Migration] and its [Migrator].
    pub approved_migration: ApprovedMigration<'info>,

    /// The existing, live program.
    pub program: LiveProgram<'info>,

    /// The [Rent] sysvar.
    pub rent: Sysvar<'info, Rent>,
    /// The [Clock] sysvar.
    pub clock: Sysvar<'info, Clock>,
    /// The [System] program.
    pub system_program: Program<'info, System>,
    /// The [bpf_loader_upgradeable] program.
    pub bpf_loader_upgradeable_program: Program<'info, BPFLoaderUpgradeable>,
}

/// Accounts for [migrator::reserve_program_id].
#[derive(Accounts)]
pub struct ReserveProgramID<'info> {
    /// Account containing the program ID.
    pub program: Signer<'info>,

    /// Address where the program data will be stored.
    pub program_data: UncheckedAccount<'info>,

    /// Payer to create the distributor.
    pub payer: Signer<'info>,

    /// The [System] program.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveMigration<'info> {
    /// The migrator.
    pub migrator: Account<'info, Migrator>,
    /// The migration.
    pub migration: Account<'info, Migration>,
    /// [Migrator::approver].
    pub approver: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(bump: u8, title: String, description: String)]
pub struct ProposeMigration<'info> {
    /// The approved [Migration] and its [Migrator].
    pub migrator: Account<'info, Migrator>,
    /// The approved [Migration] and its [Migrator].
    #[account(
        init,
        seeds = [
            b"migration".as_ref(),
            migrator.num_migrations.to_le_bytes().as_ref()
        ],
        bump = bump,
        payer = proposer,
        space = std::mem::size_of::<Migration>() + title.as_bytes().len() + description.as_bytes().len()
    )]
    pub migration: Account<'info, Migration>,
    /// The existing, live program.
    pub buffer: Account<'info, UpgradeableLoaderAccount>,
    /// The one proposing the migration. Also the payer.
    pub proposer: Signer<'info>,
    /// The [System] program.
    pub system_program: Program<'info, System>,
}

//////////////////////////////////////////
// Context structs
//////////////////////////////////////////

/// A new, undeployed program.
#[derive(Accounts)]
pub struct UndeployedProgram<'info> {
    /// Program with no data in it, owned by this program.
    pub program: UncheckedAccount<'info>,
    /// Address where the program data will be stored.
    pub program_data: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct LiveProgram<'info> {
    /// Account containing the program ID.
    pub program: Account<'info, UpgradeableLoaderAccount>,
    /// Address where the program data will be stored.
    pub program_data: Account<'info, UpgradeableLoaderAccount>,
}

#[derive(Accounts)]
pub struct ApprovedMigration<'info> {
    /// The [Migrator] associated with the program to be deployed.
    pub migrator: Account<'info, Migrator>,
    /// The [Migration] to deploy.
    pub migration: Account<'info, Migration>,
    /// Account containing the buffer to deploy.
    pub buffer: Account<'info, UpgradeableLoaderAccount>,
    /// Account which executed the deployment.
    pub executor: Signer<'info>,
}
