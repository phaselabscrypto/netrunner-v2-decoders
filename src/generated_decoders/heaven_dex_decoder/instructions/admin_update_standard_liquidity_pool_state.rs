use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x63e4293fddf4c8c7")]
pub struct AdminUpdateStandardLiquidityPoolState {
    pub update: AdminUpdateLiquidityPoolState,
}

pub struct AdminUpdateStandardLiquidityPoolStateInstructionAccounts {
    pub liquidity_pool_state: solana_sdk::pubkey::Pubkey,
    pub protocol_config: solana_sdk::pubkey::Pubkey,
    pub protocol_admin: solana_sdk::pubkey::Pubkey,
    pub protocol_admin_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminUpdateStandardLiquidityPoolState {
    type ArrangedAccounts = AdminUpdateStandardLiquidityPoolStateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let liquidity_pool_state = accounts.get(0)?;
        let protocol_config = accounts.get(1)?;
        let protocol_admin = accounts.get(2)?;
        let protocol_admin_state = accounts.get(3)?;

        Some(AdminUpdateStandardLiquidityPoolStateInstructionAccounts {
            liquidity_pool_state: liquidity_pool_state.pubkey,
            protocol_config: protocol_config.pubkey,
            protocol_admin: protocol_admin.pubkey,
            protocol_admin_state: protocol_admin_state.pubkey,
        })
    }
}
