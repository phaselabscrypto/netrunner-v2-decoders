use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2a2b7e38e70ad035")]
pub struct CreateStandardLiquidityPool {
    pub protocol_config_version: u16,
    pub params: CreateStandardLiquidityPoolParams,
}

pub struct CreateStandardLiquidityPoolInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub user_token_a_vault: solana_sdk::pubkey::Pubkey,
    pub token_a_vault: solana_sdk::pubkey::Pubkey,
    pub token_b_vault: solana_sdk::pubkey::Pubkey,
    pub liquidity_pool_state: solana_sdk::pubkey::Pubkey,
    pub protocol_config: solana_sdk::pubkey::Pubkey,
    pub token_a_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateStandardLiquidityPool {
    type ArrangedAccounts = CreateStandardLiquidityPoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let associated_token_program = accounts.get(1)?;
        let system_program = accounts.get(2)?;
        let user = accounts.get(3)?;
        let payer = accounts.get(4)?;
        let token_a_mint = accounts.get(5)?;
        let token_b_mint = accounts.get(6)?;
        let user_token_a_vault = accounts.get(7)?;
        let token_a_vault = accounts.get(8)?;
        let token_b_vault = accounts.get(9)?;
        let liquidity_pool_state = accounts.get(10)?;
        let protocol_config = accounts.get(11)?;
        let token_a_program = accounts.get(12)?;

        Some(CreateStandardLiquidityPoolInstructionAccounts {
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            user: user.pubkey,
            payer: payer.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            user_token_a_vault: user_token_a_vault.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            liquidity_pool_state: liquidity_pool_state.pubkey,
            protocol_config: protocol_config.pubkey,
            token_a_program: token_a_program.pubkey,
        })
    }
}
