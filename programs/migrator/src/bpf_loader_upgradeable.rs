use anchor_lang::{
    prelude::{ProgramError, Pubkey},
    Id, Owner,
};
use solana_program::{bpf_loader_upgradeable, declare_id, instruction::InstructionError};
use std::ops::Deref;
use vipers::try_or_err;

declare_id!("BPFLoaderUpgradeab1e11111111111111111111111");

#[derive(Clone)]
pub struct BPFLoaderUpgradeable;

impl anchor_lang::AccountDeserialize for BPFLoaderUpgradeable {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        BPFLoaderUpgradeable::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(_buf: &mut &[u8]) -> Result<Self, ProgramError> {
        Ok(BPFLoaderUpgradeable)
    }
}

impl Id for BPFLoaderUpgradeable {
    fn id() -> Pubkey {
        bpf_loader_upgradeable::ID
    }
}

/// State of an UpgradeableLoader program.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UpgradeableLoaderAccount(bpf_loader_upgradeable::UpgradeableLoaderState);

impl UpgradeableLoaderAccount {
    pub fn program_len() -> Result<usize, InstructionError> {
        bpf_loader_upgradeable::UpgradeableLoaderState::program_len()
    }
}

impl Owner for UpgradeableLoaderAccount {
    fn owner() -> Pubkey {
        ID
    }
}

impl Deref for UpgradeableLoaderAccount {
    type Target = bpf_loader_upgradeable::UpgradeableLoaderState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl anchor_lang::AccountSerialize for UpgradeableLoaderAccount {
    fn try_serialize<W: std::io::Write>(&self, _writer: &mut W) -> Result<(), ProgramError> {
        // no-op
        Ok(())
    }
}

impl anchor_lang::AccountDeserialize for UpgradeableLoaderAccount {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        UpgradeableLoaderAccount::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        let data: bpf_loader_upgradeable::UpgradeableLoaderState =
            try_or_err!(bincode::deserialize(buf), ParseError);
        Ok(UpgradeableLoaderAccount(data))
    }
}
