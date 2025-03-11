
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x9118acc2db7d03be")]
pub struct InitializeCustomizablePermissionlessConstantProductPool{
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub params: CustomizableParams,
}

pub struct InitializeCustomizablePermissionlessConstantProductPoolInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub a_vault: solana_sdk::pubkey::Pubkey,
    pub b_vault: solana_sdk::pubkey::Pubkey,
    pub a_token_vault: solana_sdk::pubkey::Pubkey,
    pub b_token_vault: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp: solana_sdk::pubkey::Pubkey,
    pub payer_token_a: solana_sdk::pubkey::Pubkey,
    pub payer_token_b: solana_sdk::pubkey::Pubkey,
    pub payer_pool_lp: solana_sdk::pubkey::Pubkey,
    pub protocol_token_a_fee: solana_sdk::pubkey::Pubkey,
    pub protocol_token_b_fee: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub mint_metadata: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
    pub vault_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeCustomizablePermissionlessConstantProductPool {
    type ArrangedAccounts = InitializeCustomizablePermissionlessConstantProductPoolInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;
        let lp_mint = accounts.get(1)?;
        let token_a_mint = accounts.get(2)?;
        let token_b_mint = accounts.get(3)?;
        let a_vault = accounts.get(4)?;
        let b_vault = accounts.get(5)?;
        let a_token_vault = accounts.get(6)?;
        let b_token_vault = accounts.get(7)?;
        let a_vault_lp_mint = accounts.get(8)?;
        let b_vault_lp_mint = accounts.get(9)?;
        let a_vault_lp = accounts.get(10)?;
        let b_vault_lp = accounts.get(11)?;
        let payer_token_a = accounts.get(12)?;
        let payer_token_b = accounts.get(13)?;
        let payer_pool_lp = accounts.get(14)?;
        let protocol_token_a_fee = accounts.get(15)?;
        let protocol_token_b_fee = accounts.get(16)?;
        let payer = accounts.get(17)?;
        let rent = accounts.get(18)?;
        let mint_metadata = accounts.get(19)?;
        let metadata_program = accounts.get(20)?;
        let vault_program = accounts.get(21)?;
        let token_program = accounts.get(22)?;
        let associated_token_program = accounts.get(23)?;
        let system_program = accounts.get(24)?;

        Some(InitializeCustomizablePermissionlessConstantProductPoolInstructionAccounts {
            pool: pool.pubkey,
            lp_mint: lp_mint.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            a_vault: a_vault.pubkey,
            b_vault: b_vault.pubkey,
            a_token_vault: a_token_vault.pubkey,
            b_token_vault: b_token_vault.pubkey,
            a_vault_lp_mint: a_vault_lp_mint.pubkey,
            b_vault_lp_mint: b_vault_lp_mint.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            b_vault_lp: b_vault_lp.pubkey,
            payer_token_a: payer_token_a.pubkey,
            payer_token_b: payer_token_b.pubkey,
            payer_pool_lp: payer_pool_lp.pubkey,
            protocol_token_a_fee: protocol_token_a_fee.pubkey,
            protocol_token_b_fee: protocol_token_b_fee.pubkey,
            payer: payer.pubkey,
            rent: rent.pubkey,
            mint_metadata: mint_metadata.pubkey,
            metadata_program: metadata_program.pubkey,
            vault_program: vault_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}