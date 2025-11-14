use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2c03a145ae4b89a2")]
pub struct GetAssetsUnderManagement {
    pub params: GetAssetsUnderManagementParams,
}

pub struct GetAssetsUnderManagementInstructionAccounts {
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetAssetsUnderManagement {
    type ArrangedAccounts = GetAssetsUnderManagementInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [perpetuals, pool, ix_sysvar, _remaining @ ..] = accounts else {
            return None;
        };

        Some(GetAssetsUnderManagementInstructionAccounts {
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
        })
    }
}
