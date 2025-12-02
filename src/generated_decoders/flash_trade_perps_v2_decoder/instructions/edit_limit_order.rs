use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2a72030b89f5ce32")]
pub struct EditLimitOrder {
    pub params: EditLimitOrderParams,
}

pub struct EditLimitOrderInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub fee_payer: solana_sdk::pubkey::Pubkey,
    pub receiving_account: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub order: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub target_custody: solana_sdk::pubkey::Pubkey,
    pub target_oracle_account: solana_sdk::pubkey::Pubkey,
    pub reserve_custody: solana_sdk::pubkey::Pubkey,
    pub reserve_oracle_account: solana_sdk::pubkey::Pubkey,
    pub reserve_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub receive_custody: solana_sdk::pubkey::Pubkey,
    pub receiving_token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
    pub receiving_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EditLimitOrder {
    type ArrangedAccounts = EditLimitOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, fee_payer, receiving_account, transfer_authority, perpetuals, pool, position, order, market, target_custody, target_oracle_account, reserve_custody, reserve_oracle_account, reserve_custody_token_account, receive_custody, receiving_token_program, event_authority, program, ix_sysvar, receiving_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(EditLimitOrderInstructionAccounts {
            owner: owner.pubkey,
            fee_payer: fee_payer.pubkey,
            receiving_account: receiving_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            order: order.pubkey,
            market: market.pubkey,
            target_custody: target_custody.pubkey,
            target_oracle_account: target_oracle_account.pubkey,
            reserve_custody: reserve_custody.pubkey,
            reserve_oracle_account: reserve_oracle_account.pubkey,
            reserve_custody_token_account: reserve_custody_token_account.pubkey,
            receive_custody: receive_custody.pubkey,
            receiving_token_program: receiving_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
            receiving_mint: receiving_mint.pubkey,
        })
    }
}
