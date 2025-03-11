
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4d55b29d3230d47e")]
pub struct InitializePermissionedPool{
    pub curve_type: CurveType,
}

pub struct InitializePermissionedPoolInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub a_vault: solana_sdk::pubkey::Pubkey,
    pub b_vault: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp: solana_sdk::pubkey::Pubkey,
    pub admin_token_a: solana_sdk::pubkey::Pubkey,
    pub admin_token_b: solana_sdk::pubkey::Pubkey,
    pub admin_pool_lp: solana_sdk::pubkey::Pubkey,
    pub protocol_token_a_fee: solana_sdk::pubkey::Pubkey,
    pub protocol_token_b_fee: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub fee_owner: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub mint_metadata: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
    pub vault_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePermissionedPool {
    type ArrangedAccounts = InitializePermissionedPoolInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;
        let lp_mint = accounts.get(1)?;
        let token_a_mint = accounts.get(2)?;
        let token_b_mint = accounts.get(3)?;
        let a_vault = accounts.get(4)?;
        let b_vault = accounts.get(5)?;
        let a_vault_lp_mint = accounts.get(6)?;
        let b_vault_lp_mint = accounts.get(7)?;
        let a_vault_lp = accounts.get(8)?;
        let b_vault_lp = accounts.get(9)?;
        let admin_token_a = accounts.get(10)?;
        let admin_token_b = accounts.get(11)?;
        let admin_pool_lp = accounts.get(12)?;
        let protocol_token_a_fee = accounts.get(13)?;
        let protocol_token_b_fee = accounts.get(14)?;
        let admin = accounts.get(15)?;
        let fee_owner = accounts.get(16)?;
        let rent = accounts.get(17)?;
        let mint_metadata = accounts.get(18)?;
        let metadata_program = accounts.get(19)?;
        let vault_program = accounts.get(20)?;
        let token_program = accounts.get(21)?;
        let associated_token_program = accounts.get(22)?;
        let system_program = accounts.get(23)?;

        Some(InitializePermissionedPoolInstructionAccounts {
            pool: pool.pubkey,
            lp_mint: lp_mint.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            a_vault: a_vault.pubkey,
            b_vault: b_vault.pubkey,
            a_vault_lp_mint: a_vault_lp_mint.pubkey,
            b_vault_lp_mint: b_vault_lp_mint.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            b_vault_lp: b_vault_lp.pubkey,
            admin_token_a: admin_token_a.pubkey,
            admin_token_b: admin_token_b.pubkey,
            admin_pool_lp: admin_pool_lp.pubkey,
            protocol_token_a_fee: protocol_token_a_fee.pubkey,
            protocol_token_b_fee: protocol_token_b_fee.pubkey,
            admin: admin.pubkey,
            fee_owner: fee_owner.pubkey,
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