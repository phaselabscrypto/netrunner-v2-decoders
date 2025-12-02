use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5055d14818ceb16c")]
pub struct RemoveLiquidity {
    pub params: RemoveLiquidityParams,
}

pub struct RemoveLiquidityInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub receiving_account: solana_sdk::pubkey::Pubkey,
    pub lp_token_account: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub custody_token_account: solana_sdk::pubkey::Pubkey,
    pub lp_token_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
    pub receiving_mint: solana_sdk::pubkey::Pubkey,
    pub receiving_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveLiquidity {
    type ArrangedAccounts = RemoveLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, receiving_account, lp_token_account, transfer_authority, perpetuals, pool, custody, custody_oracle_account, custody_token_account, lp_token_mint, token_program, event_authority, program, ix_sysvar, receiving_mint, receiving_token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RemoveLiquidityInstructionAccounts {
            owner: owner.pubkey,
            receiving_account: receiving_account.pubkey,
            lp_token_account: lp_token_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            custody_oracle_account: custody_oracle_account.pubkey,
            custody_token_account: custody_token_account.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
            receiving_mint: receiving_mint.pubkey,
            receiving_token_program: receiving_token_program.pubkey,
        })
    }
}
