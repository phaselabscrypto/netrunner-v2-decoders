

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1513d02bed3eff57")]
pub struct Lock{
    pub max_amount: u64,
}

pub struct LockInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub lock_escrow: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub source_tokens: solana_sdk::pubkey::Pubkey,
    pub escrow_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub a_vault: solana_sdk::pubkey::Pubkey,
    pub b_vault: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Lock {
    type ArrangedAccounts = LockInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;
        let lp_mint = accounts.get(1)?;
        let lock_escrow = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let source_tokens = accounts.get(4)?;
        let escrow_vault = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let a_vault = accounts.get(7)?;
        let b_vault = accounts.get(8)?;
        let a_vault_lp = accounts.get(9)?;
        let b_vault_lp = accounts.get(10)?;
        let a_vault_lp_mint = accounts.get(11)?;
        let b_vault_lp_mint = accounts.get(12)?;

        Some(LockInstructionAccounts {
            pool: pool.pubkey,
            lp_mint: lp_mint.pubkey,
            lock_escrow: lock_escrow.pubkey,
            owner: owner.pubkey,
            source_tokens: source_tokens.pubkey,
            escrow_vault: escrow_vault.pubkey,
            token_program: token_program.pubkey,
            a_vault: a_vault.pubkey,
            b_vault: b_vault.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            b_vault_lp: b_vault_lp.pubkey,
            a_vault_lp_mint: a_vault_lp_mint.pubkey,
            b_vault_lp_mint: b_vault_lp_mint.pubkey,
        })
    }
}