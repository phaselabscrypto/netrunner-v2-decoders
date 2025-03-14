

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe0e2df50243246e7")]
pub struct FillDynamicAmm{
    pub max_amount: u64,
}

pub struct FillDynamicAmmInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_out_vault: solana_sdk::pubkey::Pubkey,
    pub amm_program: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub a_vault: solana_sdk::pubkey::Pubkey,
    pub b_vault: solana_sdk::pubkey::Pubkey,
    pub a_token_vault: solana_sdk::pubkey::Pubkey,
    pub b_token_vault: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp: solana_sdk::pubkey::Pubkey,
    pub admin_token_fee: solana_sdk::pubkey::Pubkey,
    pub vault_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FillDynamicAmm {
    type ArrangedAccounts = FillDynamicAmmInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let token_vault = accounts.get(1)?;
        let token_out_vault = accounts.get(2)?;
        let amm_program = accounts.get(3)?;
        let pool = accounts.get(4)?;
        let a_vault = accounts.get(5)?;
        let b_vault = accounts.get(6)?;
        let a_token_vault = accounts.get(7)?;
        let b_token_vault = accounts.get(8)?;
        let a_vault_lp_mint = accounts.get(9)?;
        let b_vault_lp_mint = accounts.get(10)?;
        let a_vault_lp = accounts.get(11)?;
        let b_vault_lp = accounts.get(12)?;
        let admin_token_fee = accounts.get(13)?;
        let vault_program = accounts.get(14)?;
        let token_program = accounts.get(15)?;
        let event_authority = accounts.get(16)?;
        let program = accounts.get(17)?;

        Some(FillDynamicAmmInstructionAccounts {
            vault: vault.pubkey,
            token_vault: token_vault.pubkey,
            token_out_vault: token_out_vault.pubkey,
            amm_program: amm_program.pubkey,
            pool: pool.pubkey,
            a_vault: a_vault.pubkey,
            b_vault: b_vault.pubkey,
            a_token_vault: a_token_vault.pubkey,
            b_token_vault: b_token_vault.pubkey,
            a_vault_lp_mint: a_vault_lp_mint.pubkey,
            b_vault_lp_mint: b_vault_lp_mint.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            b_vault_lp: b_vault_lp.pubkey,
            admin_token_fee: admin_token_fee.pubkey,
            vault_program: vault_program.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}