use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf8c69e91e17587c8")]
pub struct Swap {
    pub params: SwapParams,
}

pub struct SwapInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub funding_account: solana_sdk::pubkey::Pubkey,
    pub receiving_account: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub receiving_custody: solana_sdk::pubkey::Pubkey,
    pub receiving_custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub receiving_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub dispensing_custody: solana_sdk::pubkey::Pubkey,
    pub dispensing_custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub dispensing_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
    pub funding_mint: solana_sdk::pubkey::Pubkey,
    pub funding_token_program: solana_sdk::pubkey::Pubkey,
    pub receiving_mint: solana_sdk::pubkey::Pubkey,
    pub receiving_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Swap {
    type ArrangedAccounts = SwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, funding_account, receiving_account, transfer_authority, perpetuals, pool, receiving_custody, receiving_custody_oracle_account, receiving_custody_token_account, dispensing_custody, dispensing_custody_oracle_account, dispensing_custody_token_account, event_authority, program, ix_sysvar, funding_mint, funding_token_program, receiving_mint, receiving_token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapInstructionAccounts {
            owner: owner.pubkey,
            funding_account: funding_account.pubkey,
            receiving_account: receiving_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            receiving_custody: receiving_custody.pubkey,
            receiving_custody_oracle_account: receiving_custody_oracle_account.pubkey,
            receiving_custody_token_account: receiving_custody_token_account.pubkey,
            dispensing_custody: dispensing_custody.pubkey,
            dispensing_custody_oracle_account: dispensing_custody_oracle_account.pubkey,
            dispensing_custody_token_account: dispensing_custody_token_account.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
            funding_mint: funding_mint.pubkey,
            funding_token_program: funding_token_program.pubkey,
            receiving_mint: receiving_mint.pubkey,
            receiving_token_program: receiving_token_program.pubkey,
        })
    }
}
