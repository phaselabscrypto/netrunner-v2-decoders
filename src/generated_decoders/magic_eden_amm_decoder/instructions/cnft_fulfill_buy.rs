
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x91ade944244cc309")]
pub struct CnftFulfillBuy{
    pub args: SolCnftFulfillBuyArgs,
}

pub struct CnftFulfillBuyInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub referral: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub buyside_sol_escrow_account: solana_sdk::pubkey::Pubkey,
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
    pub bubblegum_program: solana_sdk::pubkey::Pubkey,
    pub compression_program: solana_sdk::pubkey::Pubkey,
    pub sell_state: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CnftFulfillBuy {
    type ArrangedAccounts = CnftFulfillBuyInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let cosigner = accounts.get(2)?;
        let referral = accounts.get(3)?;
        let pool = accounts.get(4)?;
        let buyside_sol_escrow_account = accounts.get(5)?;
        let tree_authority = accounts.get(6)?;
        let merkle_tree = accounts.get(7)?;
        let log_wrapper = accounts.get(8)?;
        let bubblegum_program = accounts.get(9)?;
        let compression_program = accounts.get(10)?;
        let sell_state = accounts.get(11)?;
        let system_program = accounts.get(12)?;

        Some(CnftFulfillBuyInstructionAccounts {
            payer: payer.pubkey,
            owner: owner.pubkey,
            cosigner: cosigner.pubkey,
            referral: referral.pubkey,
            pool: pool.pubkey,
            buyside_sol_escrow_account: buyside_sol_escrow_account.pubkey,
            tree_authority: tree_authority.pubkey,
            merkle_tree: merkle_tree.pubkey,
            log_wrapper: log_wrapper.pubkey,
            bubblegum_program: bubblegum_program.pubkey,
            compression_program: compression_program.pubkey,
            sell_state: sell_state.pubkey,
            system_program: system_program.pubkey,
        })
    }
}