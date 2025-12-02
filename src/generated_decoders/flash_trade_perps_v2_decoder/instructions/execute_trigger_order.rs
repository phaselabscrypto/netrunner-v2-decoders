use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x690a6888d78654ab")]
pub struct ExecuteTriggerOrder {
    pub params: ExecuteTriggerOrderParams,
}

pub struct ExecuteTriggerOrderInstructionAccounts {
    pub position_owner: solana_sdk::pubkey::Pubkey,
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
    pub collateral_custody: solana_sdk::pubkey::Pubkey,
    pub collateral_oracle_account: solana_sdk::pubkey::Pubkey,
    pub collateral_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub receiving_token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
    pub receiving_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ExecuteTriggerOrder {
    type ArrangedAccounts = ExecuteTriggerOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position_owner, fee_payer, receiving_account, transfer_authority, perpetuals, pool, position, order, market, target_custody, target_oracle_account, collateral_custody, collateral_oracle_account, collateral_custody_token_account, receiving_token_program, event_authority, program, ix_sysvar, receiving_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ExecuteTriggerOrderInstructionAccounts {
            position_owner: position_owner.pubkey,
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
            collateral_custody: collateral_custody.pubkey,
            collateral_oracle_account: collateral_oracle_account.pubkey,
            collateral_custody_token_account: collateral_custody_token_account.pubkey,
            receiving_token_program: receiving_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
            receiving_mint: receiving_mint.pubkey,
        })
    }
}
