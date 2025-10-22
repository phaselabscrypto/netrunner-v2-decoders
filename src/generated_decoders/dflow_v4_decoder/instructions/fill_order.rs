use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe87a7319c78f88a2")]
pub struct FillOrder {
    pub params: FillOrderParams,
}

pub struct FillOrderInstructionAccounts {
    pub order: solana_sdk::pubkey::Pubkey,
    pub order_vault: solana_sdk::pubkey::Pubkey,
    pub output_token_account: solana_sdk::pubkey::Pubkey,
    pub return_input_token_account: solana_sdk::pubkey::Pubkey,
    pub return_rent_to: solana_sdk::pubkey::Pubkey,
    pub filler_input_token_account: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub filler_output_token_account: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub filler: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FillOrder {
    type ArrangedAccounts = FillOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let order = accounts.get(0)?;
        let order_vault = accounts.get(1)?;
        let output_token_account = accounts.get(2)?;
        let return_input_token_account = accounts.get(3)?;
        let return_rent_to = accounts.get(4)?;
        let filler_input_token_account = accounts.get(5)?;
        let input_mint = accounts.get(6)?;
        let filler_output_token_account = accounts.get(7)?;
        let output_mint = accounts.get(8)?;
        let filler = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let associated_token_program = accounts.get(11)?;
        let system_program = accounts.get(12)?;
        let event_authority = accounts.get(13)?;
        let program = accounts.get(14)?;

        Some(FillOrderInstructionAccounts {
            order: order.pubkey,
            order_vault: order_vault.pubkey,
            output_token_account: output_token_account.pubkey,
            return_input_token_account: return_input_token_account.pubkey,
            return_rent_to: return_rent_to.pubkey,
            filler_input_token_account: filler_input_token_account.pubkey,
            input_mint: input_mint.pubkey,
            filler_output_token_account: filler_output_token_account.pubkey,
            output_mint: output_mint.pubkey,
            filler: filler.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
