use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xce58588f268832e0")]
pub struct OpenOrder {
    pub params: OpenOrderParams,
}

pub struct OpenOrderInstructionAccounts {
    pub order: solana_sdk::pubkey::Pubkey,
    pub order_vault: solana_sdk::pubkey::Pubkey,
    pub input_token_account: solana_sdk::pubkey::Pubkey,
    pub output_token_account: solana_sdk::pubkey::Pubkey,
    pub return_input_token_account: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub user_token_authority: solana_sdk::pubkey::Pubkey,
    pub fee_payer: solana_sdk::pubkey::Pubkey,
    pub fee_receiver: solana_sdk::pubkey::Pubkey,
    pub rent_depositor: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OpenOrder {
    type ArrangedAccounts = OpenOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let order = accounts.get(0)?;
        let order_vault = accounts.get(1)?;
        let input_token_account = accounts.get(2)?;
        let output_token_account = accounts.get(3)?;
        let return_input_token_account = accounts.get(4)?;
        let input_mint = accounts.get(5)?;
        let user_token_authority = accounts.get(6)?;
        let fee_payer = accounts.get(7)?;
        let fee_receiver = accounts.get(8)?;
        let rent_depositor = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let system_program = accounts.get(11)?;
        let rent = accounts.get(12)?;

        Some(OpenOrderInstructionAccounts {
            order: order.pubkey,
            order_vault: order_vault.pubkey,
            input_token_account: input_token_account.pubkey,
            output_token_account: output_token_account.pubkey,
            return_input_token_account: return_input_token_account.pubkey,
            input_mint: input_mint.pubkey,
            user_token_authority: user_token_authority.pubkey,
            fee_payer: fee_payer.pubkey,
            fee_receiver: fee_receiver.pubkey,
            rent_depositor: rent_depositor.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
