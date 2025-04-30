use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe4bfce0cf25a8922")]
pub struct SaberAddDecimalsWithdraw {
    pub in_amount: Option<u64>,
    pub minimum_out_amount: u64,
    pub platform_fee_bps: u8,
}

pub struct SaberAddDecimalsWithdrawInstructionAccounts {
    pub add_decimals_program: solana_sdk::pubkey::Pubkey,
    pub wrapper: solana_sdk::pubkey::Pubkey,
    pub wrapper_mint: solana_sdk::pubkey::Pubkey,
    pub wrapper_underlying_tokens: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub user_underlying_tokens: solana_sdk::pubkey::Pubkey,
    pub user_wrapped_tokens: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SaberAddDecimalsWithdraw {
    type ArrangedAccounts = SaberAddDecimalsWithdrawInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let add_decimals_program = accounts.get(0)?;
        let wrapper = accounts.get(1)?;
        let wrapper_mint = accounts.get(2)?;
        let wrapper_underlying_tokens = accounts.get(3)?;
        let owner = accounts.get(4)?;
        let user_underlying_tokens = accounts.get(5)?;
        let user_wrapped_tokens = accounts.get(6)?;
        let token_program = accounts.get(7)?;

        Some(SaberAddDecimalsWithdrawInstructionAccounts {
            add_decimals_program: add_decimals_program.pubkey,
            wrapper: wrapper.pubkey,
            wrapper_mint: wrapper_mint.pubkey,
            wrapper_underlying_tokens: wrapper_underlying_tokens.pubkey,
            owner: owner.pubkey,
            user_underlying_tokens: user_underlying_tokens.pubkey,
            user_wrapped_tokens: user_wrapped_tokens.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
