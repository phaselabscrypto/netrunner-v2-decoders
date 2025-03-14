

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa9204f8988e84689")]
pub struct ClaimFee{
    pub max_amount: u64,
}

pub struct ClaimFeeInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub lock_escrow: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub source_tokens: solana_sdk::pubkey::Pubkey,
    pub escrow_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub a_token_vault: solana_sdk::pubkey::Pubkey,
    pub b_token_vault: solana_sdk::pubkey::Pubkey,
    pub a_vault: solana_sdk::pubkey::Pubkey,
    pub b_vault: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub user_a_token: solana_sdk::pubkey::Pubkey,
    pub user_b_token: solana_sdk::pubkey::Pubkey,
    pub vault_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimFee {
    type ArrangedAccounts = ClaimFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;
        let lp_mint = accounts.get(1)?;
        let lock_escrow = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let source_tokens = accounts.get(4)?;
        let escrow_vault = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let a_token_vault = accounts.get(7)?;
        let b_token_vault = accounts.get(8)?;
        let a_vault = accounts.get(9)?;
        let b_vault = accounts.get(10)?;
        let a_vault_lp = accounts.get(11)?;
        let b_vault_lp = accounts.get(12)?;
        let a_vault_lp_mint = accounts.get(13)?;
        let b_vault_lp_mint = accounts.get(14)?;
        let user_a_token = accounts.get(15)?;
        let user_b_token = accounts.get(16)?;
        let vault_program = accounts.get(17)?;

        Some(ClaimFeeInstructionAccounts {
            pool: pool.pubkey,
            lp_mint: lp_mint.pubkey,
            lock_escrow: lock_escrow.pubkey,
            owner: owner.pubkey,
            source_tokens: source_tokens.pubkey,
            escrow_vault: escrow_vault.pubkey,
            token_program: token_program.pubkey,
            a_token_vault: a_token_vault.pubkey,
            b_token_vault: b_token_vault.pubkey,
            a_vault: a_vault.pubkey,
            b_vault: b_vault.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            b_vault_lp: b_vault_lp.pubkey,
            a_vault_lp_mint: a_vault_lp_mint.pubkey,
            b_vault_lp_mint: b_vault_lp_mint.pubkey,
            user_a_token: user_a_token.pubkey,
            user_b_token: user_b_token.pubkey,
            vault_program: vault_program.pubkey,
        })
    }
}