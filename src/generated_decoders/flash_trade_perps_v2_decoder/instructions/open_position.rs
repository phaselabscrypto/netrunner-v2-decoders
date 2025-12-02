use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x87802f4d0f98f031")]
pub struct OpenPosition {
    pub params: OpenPositionParams,
}

pub struct OpenPositionInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub fee_payer: solana_sdk::pubkey::Pubkey,
    pub funding_account: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub target_custody: solana_sdk::pubkey::Pubkey,
    pub target_oracle_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody: solana_sdk::pubkey::Pubkey,
    pub collateral_oracle_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub funding_token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
    pub funding_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OpenPosition {
    type ArrangedAccounts = OpenPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, fee_payer, funding_account, transfer_authority, perpetuals, pool, position, market, target_custody, target_oracle_account, collateral_custody, collateral_oracle_account, collateral_custody_token_account, system_program, funding_token_program, event_authority, program, ix_sysvar, funding_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(OpenPositionInstructionAccounts {
            owner: owner.pubkey,
            fee_payer: fee_payer.pubkey,
            funding_account: funding_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            market: market.pubkey,
            target_custody: target_custody.pubkey,
            target_oracle_account: target_oracle_account.pubkey,
            collateral_custody: collateral_custody.pubkey,
            collateral_oracle_account: collateral_oracle_account.pubkey,
            collateral_custody_token_account: collateral_custody_token_account.pubkey,
            system_program: system_program.pubkey,
            funding_token_program: funding_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
            funding_mint: funding_mint.pubkey,
        })
    }
}
