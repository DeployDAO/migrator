use anchor_lang::prelude::*;
use solana_program::{
    bpf_loader_upgradeable::{self, UpgradeableLoaderState},
    system_program,
};
use vipers::{assert_keys, invariant, program_err, unwrap_opt, validate::Validate};

use crate::{
    account_contexts::{NewMigrator, RejectMigration, ReserveProgramID},
    bpf_loader_upgradeable::UpgradeableLoaderAccount,
    ApproveMigration, ApprovedMigration, DeployProgram, LiveProgram, ProposeMigration,
    UndeployedProgram, UpgradeProgram,
};

impl<'info> Validate<'info> for NewMigrator<'info> {
    fn validate(&self) -> ProgramResult {
        let migrator_key = self.migrator.key();
        let program = &self.program;
        let program_data = &self.program_data;

        if program_data.data_is_empty() {
            // migrator for an undeployed program
            (UndeployedProgram {
                program: self.program.clone(),
                program_data: self.program_data.clone(),
            })
            .validate_for_migrator(migrator_key)?;
        } else {
            // migrator for a live program
            let program: Account<UpgradeableLoaderAccount> = Account::try_from(program)?;
            let program_data: Account<UpgradeableLoaderAccount> = Account::try_from(program_data)?;
            (LiveProgram {
                program,
                program_data,
            })
            .validate_for_migrator(migrator_key)?;
        }

        Ok(())
    }
}

impl<'info> Validate<'info> for DeployProgram<'info> {
    fn validate(&self) -> ProgramResult {
        self.approved_migration.validate()?;
        self.program
            .validate_for_migrator(self.approved_migration.migrator.key())?;

        assert_keys!(
            self.approved_migration.migrator.program_id,
            self.program.program,
            "approved_migration.migrator.program_id"
        );

        Ok(())
    }
}

impl<'info> Validate<'info> for UpgradeProgram<'info> {
    fn validate(&self) -> ProgramResult {
        self.approved_migration.validate()?;
        self.program
            .validate_for_migrator(self.approved_migration.migrator.key())?;
        assert_keys!(
            self.approved_migration.migrator.program_id,
            self.program.program,
            "approved_migration.migrator.program_id"
        );

        Ok(())
    }
}

impl<'info> Validate<'info> for ApproveMigration<'info> {
    fn validate(&self) -> ProgramResult {
        assert_keys!(
            self.migration.migrator,
            self.migration,
            "migration.migrator"
        );
        assert_keys!(self.migrator.approver, self.approver, "migrator.approver");
        require!(self.migration.executed_at == -1, MigrationAlreadyExecuted);

        Ok(())
    }
}

impl<'info> Validate<'info> for RejectMigration<'info> {
    fn validate(&self) -> ProgramResult {
        assert_keys!(
            self.migration.migrator,
            self.migration,
            "migration.migrator"
        );
        assert_keys!(self.migrator.approver, self.approver, "migrator.approver");
        require!(self.migration.executed_at == -1, MigrationAlreadyExecuted);

        Ok(())
    }
}

impl<'info> Validate<'info> for ProposeMigration<'info> {
    fn validate(&self) -> ProgramResult {
        if let UpgradeableLoaderState::Buffer { authority_address } = **self.buffer {
            assert_keys!(
                unwrap_opt!(authority_address, "no buffer authority"),
                self.migrator,
                "buffer authority must be migrator"
            );
        } else {
            return program_err!(BufferAuthorityMismatch);
        }
        Ok(())
    }
}

impl<'info> Validate<'info> for ReserveProgramID<'info> {
    fn validate(&self) -> ProgramResult {
        assert_keys!(
            *self.program.to_account_info().owner,
            system_program::ID,
            "program must not be a program account"
        );
        Ok(())
    }
}

impl<'info> Validate<'info> for ApprovedMigration<'info> {
    fn validate(&self) -> ProgramResult {
        assert_keys!(
            self.migrator.pending_migration,
            self.migration,
            "migrator.pending_migration"
        );
        assert_keys!(self.migration.migrator, self.migrator, "migration.migrator");
        assert_keys!(self.migration.buffer, self.buffer, "migration.buffer");

        if let UpgradeableLoaderState::Buffer { authority_address } = **self.buffer {
            assert_keys!(
                unwrap_opt!(authority_address, "no buffer authority"),
                self.migrator,
                "buffer authority must be migrator"
            );
        } else {
            return program_err!(BufferAuthorityMismatch);
        }

        assert_keys!(self.migration.buffer, self.buffer, "migration.buffer");

        assert_keys!(
            self.migrator.pending_migration,
            self.migration,
            "pending_migration"
        );
        let migrator = &self.migrator;
        let now = Clock::get()?.unix_timestamp;
        require!(migrator.approval_expires_at != -1, NoApprovedMigration);
        require!(migrator.approval_expires_at < now, MigrationWindowExpired);

        require!(self.migration.rejected_at != -1, MigrationRejected);
        require!(self.migration.executed_at == -1, MigrationAlreadyExecuted);

        Ok(())
    }
}

impl<'info> UndeployedProgram<'info> {
    pub fn validate_for_migrator(&self, migrator: Pubkey) -> ProgramResult {
        let program = &self.program;
        let program_data = &self.program_data;

        let (migrator_address, _) = Pubkey::find_program_address(
            &[b"migrator".as_ref(), &program.key().to_bytes()],
            &crate::ID,
        );
        assert_keys!(migrator, migrator_address, "migrator should be canonical");

        let (programdata_address, _) =
            Pubkey::find_program_address(&[program.key().as_ref()], &bpf_loader_upgradeable::ID);
        assert_keys!(
            programdata_address,
            program_data.key(),
            "programdata_address"
        );

        assert_keys!(
            *program.owner,
            crate::ID,
            "program must be owned by this program"
        );
        invariant!(program_data.data_is_empty(), "program data must be empty");

        Ok(())
    }
}

impl<'info> LiveProgram<'info> {
    pub fn validate_for_migrator(&self, migrator: Pubkey) -> ProgramResult {
        let program = &self.program;
        let program_data = &self.program_data;

        if let (
            UpgradeableLoaderState::Program {
                programdata_address,
            },
            UpgradeableLoaderState::ProgramData {
                slot: _,
                upgrade_authority_address,
            },
        ) = (***program, ***program_data)
        {
            assert_keys!(programdata_address, *program_data, "programdata_address");
            assert_keys!(
                unwrap_opt!(upgrade_authority_address, "upgrade_authority must be set"),
                migrator,
                "upgrade_authority must be migrator"
            );
        } else {
            return program_err!(ParseError);
        }

        Ok(())
    }
}
