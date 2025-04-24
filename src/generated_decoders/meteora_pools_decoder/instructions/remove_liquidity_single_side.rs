use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5454b142feb90afb")]
pub struct RemoveLiquiditySingleSide {
    pub pool_token_amount: u64,
    pub minimum_out_amount: u64,
}

pub struct RemoveLiquiditySingleSideInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub user_pool_lp: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp: solana_sdk::pubkey::Pubkey,
    pub a_vault: solana_sdk::pubkey::Pubkey,
    pub b_vault: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub a_token_vault: solana_sdk::pubkey::Pubkey,
    pub b_token_vault: solana_sdk::pubkey::Pubkey,
    pub user_destination_token: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub vault_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveLiquiditySingleSide {
    type ArrangedAccounts = RemoveLiquiditySingleSideInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;
        let lp_mint = accounts.get(1)?;
        let user_pool_lp = accounts.get(2)?;
        let a_vault_lp = accounts.get(3)?;
        let b_vault_lp = accounts.get(4)?;
        let a_vault = accounts.get(5)?;
        let b_vault = accounts.get(6)?;
        let a_vault_lp_mint = accounts.get(7)?;
        let b_vault_lp_mint = accounts.get(8)?;
        let a_token_vault = accounts.get(9)?;
        let b_token_vault = accounts.get(10)?;
        let user_destination_token = accounts.get(11)?;
        let user = accounts.get(12)?;
        let vault_program = accounts.get(13)?;
        let token_program = accounts.get(14)?;

        Some(RemoveLiquiditySingleSideInstructionAccounts {
            pool: pool.pubkey,
            lp_mint: lp_mint.pubkey,
            user_pool_lp: user_pool_lp.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            b_vault_lp: b_vault_lp.pubkey,
            a_vault: a_vault.pubkey,
            b_vault: b_vault.pubkey,
            a_vault_lp_mint: a_vault_lp_mint.pubkey,
            b_vault_lp_mint: b_vault_lp_mint.pubkey,
            a_token_vault: a_token_vault.pubkey,
            b_token_vault: b_token_vault.pubkey,
            user_destination_token: user_destination_token.pubkey,
            user: user.pubkey,
            vault_program: vault_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
