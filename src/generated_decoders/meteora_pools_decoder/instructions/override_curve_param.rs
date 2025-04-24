use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6256cc335e4745bb")]
pub struct OverrideCurveParam {
    pub curve_type: CurveType,
}

pub struct OverrideCurveParamInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OverrideCurveParam {
    type ArrangedAccounts = OverrideCurveParamInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;
        let admin = accounts.get(1)?;

        Some(OverrideCurveParamInstructionAccounts {
            pool: pool.pubkey,
            admin: admin.pubkey,
        })
    }
}
