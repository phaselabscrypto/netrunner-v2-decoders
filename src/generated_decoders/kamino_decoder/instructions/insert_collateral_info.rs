use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1661044ea6bc33be")]
pub struct InsertCollateralInfo {
    pub index: u64,
    pub params: CollateralInfoParams,
}

pub struct InsertCollateralInfoInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub token_infos: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InsertCollateralInfo {
    type ArrangedAccounts = InsertCollateralInfoInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let global_config = accounts.get(1)?;
        let token_infos = accounts.get(2)?;

        Some(InsertCollateralInfoInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            global_config: global_config.pubkey,
            token_infos: token_infos.pubkey,
        })
    }
}
