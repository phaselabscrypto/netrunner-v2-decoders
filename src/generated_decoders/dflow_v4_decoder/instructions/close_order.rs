use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5a67d11c073fa804")]
pub struct CloseOrder {}

pub struct CloseOrderInstructionAccounts {
    pub order: solana_sdk::pubkey::Pubkey,
    pub order_vault: solana_sdk::pubkey::Pubkey,
    pub return_input_token_account: solana_sdk::pubkey::Pubkey,
    pub return_rent_to: solana_sdk::pubkey::Pubkey,
    pub closer: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseOrder {
    type ArrangedAccounts = CloseOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let order = accounts.get(0)?;
        let order_vault = accounts.get(1)?;
        let return_input_token_account = accounts.get(2)?;
        let return_rent_to = accounts.get(3)?;
        let closer = accounts.get(4)?;
        let token_program = accounts.get(5)?;
        let system_program = accounts.get(6)?;

        Some(CloseOrderInstructionAccounts {
            order: order.pubkey,
            order_vault: order_vault.pubkey,
            return_input_token_account: return_input_token_account.pubkey,
            return_rent_to: return_rent_to.pubkey,
            closer: closer.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
