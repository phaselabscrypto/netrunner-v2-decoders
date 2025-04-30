use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfbe877a6e1b9a9a1")]
pub struct AldrinSwap {}

pub struct AldrinSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub pool_signer: solana_sdk::pubkey::Pubkey,
    pub pool_mint: solana_sdk::pubkey::Pubkey,
    pub base_token_vault: solana_sdk::pubkey::Pubkey,
    pub quote_token_vault: solana_sdk::pubkey::Pubkey,
    pub fee_pool_token_account: solana_sdk::pubkey::Pubkey,
    pub wallet_authority: solana_sdk::pubkey::Pubkey,
    pub user_base_token_account: solana_sdk::pubkey::Pubkey,
    pub user_quote_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AldrinSwap {
    type ArrangedAccounts = AldrinSwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let pool_signer = accounts.get(2)?;
        let pool_mint = accounts.get(3)?;
        let base_token_vault = accounts.get(4)?;
        let quote_token_vault = accounts.get(5)?;
        let fee_pool_token_account = accounts.get(6)?;
        let wallet_authority = accounts.get(7)?;
        let user_base_token_account = accounts.get(8)?;
        let user_quote_token_account = accounts.get(9)?;
        let token_program = accounts.get(10)?;

        Some(AldrinSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            pool: pool.pubkey,
            pool_signer: pool_signer.pubkey,
            pool_mint: pool_mint.pubkey,
            base_token_vault: base_token_vault.pubkey,
            quote_token_vault: quote_token_vault.pubkey,
            fee_pool_token_account: fee_pool_token_account.pubkey,
            wallet_authority: wallet_authority.pubkey,
            user_base_token_account: user_base_token_account.pubkey,
            user_quote_token_account: user_quote_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
