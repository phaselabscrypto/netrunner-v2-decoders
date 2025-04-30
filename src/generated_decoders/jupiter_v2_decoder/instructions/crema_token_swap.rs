use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xeba0af7a3db102f7")]
pub struct CremaTokenSwap {
    pub in_amount: Option<u64>,
    pub minimum_out_amount: u64,
    pub platform_fee_bps: u8,
}

pub struct CremaTokenSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub pool_signer: solana_sdk::pubkey::Pubkey,
    pub user_source_token_account: solana_sdk::pubkey::Pubkey,
    pub user_destination_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_source_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_destination_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_ticks_account: solana_sdk::pubkey::Pubkey,
    pub wallet_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CremaTokenSwap {
    type ArrangedAccounts = CremaTokenSwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let pool_signer = accounts.get(2)?;
        let user_source_token_account = accounts.get(3)?;
        let user_destination_token_account = accounts.get(4)?;
        let pool_source_token_account = accounts.get(5)?;
        let pool_destination_token_account = accounts.get(6)?;
        let pool_ticks_account = accounts.get(7)?;
        let wallet_authority = accounts.get(8)?;
        let token_program = accounts.get(9)?;

        Some(CremaTokenSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            pool: pool.pubkey,
            pool_signer: pool_signer.pubkey,
            user_source_token_account: user_source_token_account.pubkey,
            user_destination_token_account: user_destination_token_account.pubkey,
            pool_source_token_account: pool_source_token_account.pubkey,
            pool_destination_token_account: pool_destination_token_account.pubkey,
            pool_ticks_account: pool_ticks_account.pubkey,
            wallet_authority: wallet_authority.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
