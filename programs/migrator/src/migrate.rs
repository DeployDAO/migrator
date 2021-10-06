use crate::ApprovedMigration;
use anchor_lang::prelude::*;

impl<'info> ApprovedMigration<'info> {
    /// Commit the result of a successful migration.
    pub fn commit(&mut self) -> ProgramResult {
        let migration = &mut self.migration;
        migration.executed_at = Clock::get()?.unix_timestamp;
        migration.executor = self.executor.key();

        let migrator = &mut self.migrator;
        migrator.pending_migration = Pubkey::default();
        migrator.approval_expires_at = -1;
        migrator.latest_migration_index = migration.index;

        // ensure we still have enough lamports for rent exemption
        let rent = Rent::get()?;
        self.migrator.reload()?;
        let migrator_info: AccountInfo = self.migrator.to_account_info();
        require!(
            rent.is_exempt(migrator_info.lamports(), migrator_info.data_len()),
            InsufficientLamports
        );

        Ok(())
    }
}
