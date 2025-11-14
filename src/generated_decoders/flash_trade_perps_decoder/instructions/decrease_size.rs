use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xab1ccb1d7610d6a9")]
pub struct DecreaseSize {
    pub params: DecreaseSizeParams,
}

pub struct DecreaseSizeInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
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
    pub collateral_token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
    pub collateral_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DecreaseSize {
    type ArrangedAccounts = DecreaseSizeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, transfer_authority, perpetuals, pool, position, market, target_custody, target_oracle_account, collateral_custody, collateral_oracle_account, collateral_custody_token_account, collateral_token_program, event_authority, program, ix_sysvar, collateral_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DecreaseSizeInstructionAccounts {
            owner: owner.pubkey,
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
            collateral_token_program: collateral_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
            collateral_mint: collateral_mint.pubkey,
        })
    }
}
