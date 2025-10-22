use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd16c20f69dd6ed56")]
pub struct InitAmmAdminGroup {
    pub params: InitAdminGroupParams,
}

pub struct InitAmmAdminGroupInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub admin_group: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitAmmAdminGroup {
    type ArrangedAccounts = InitAmmAdminGroupInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let admin_group = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(InitAmmAdminGroupInstructionAccounts {
            payer: payer.pubkey,
            admin_group: admin_group.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
