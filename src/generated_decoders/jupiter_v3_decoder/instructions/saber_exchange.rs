use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x919eb8d4034a9c76")]
pub struct SaberExchange {
    pub in_amount: Option<u64>,
    pub minimum_out_amount: u64,
    pub platform_fee_bps: u8,
}

pub struct SaberExchangeInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub swap: solana_sdk::pubkey::Pubkey,
    pub swap_authority: solana_sdk::pubkey::Pubkey,
    pub user_authority: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub input_user_account: solana_sdk::pubkey::Pubkey,
    pub input_token_account: solana_sdk::pubkey::Pubkey,
    pub output_user_account: solana_sdk::pubkey::Pubkey,
    pub output_token_account: solana_sdk::pubkey::Pubkey,
    pub fees_token_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SaberExchange {
    type ArrangedAccounts = SaberExchangeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let token_program = accounts.get(1)?;
        let swap = accounts.get(2)?;
        let swap_authority = accounts.get(3)?;
        let user_authority = accounts.get(4)?;
        let clock = accounts.get(5)?;
        let input_user_account = accounts.get(6)?;
        let input_token_account = accounts.get(7)?;
        let output_user_account = accounts.get(8)?;
        let output_token_account = accounts.get(9)?;
        let fees_token_account = accounts.get(10)?;

        Some(SaberExchangeInstructionAccounts {
            swap_program: swap_program.pubkey,
            token_program: token_program.pubkey,
            swap: swap.pubkey,
            swap_authority: swap_authority.pubkey,
            user_authority: user_authority.pubkey,
            clock: clock.pubkey,
            input_user_account: input_user_account.pubkey,
            input_token_account: input_token_account.pubkey,
            output_user_account: output_user_account.pubkey,
            output_token_account: output_token_account.pubkey,
            fees_token_account: fees_token_account.pubkey,
        })
    }
}
