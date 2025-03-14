
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xfce7c9b01ed57612")]
pub struct SolMplCoreFulfillSell{
    pub args: SolMplCoreFulfillSellArgs,
}

pub struct SolMplCoreFulfillSellInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub referral: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub buyside_sol_escrow_account: solana_sdk::pubkey::Pubkey,
    pub asset: solana_sdk::pubkey::Pubkey,
    pub sell_state: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub asset_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SolMplCoreFulfillSell {
    type ArrangedAccounts = SolMplCoreFulfillSellInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let cosigner = accounts.get(2)?;
        let referral = accounts.get(3)?;
        let pool = accounts.get(4)?;
        let buyside_sol_escrow_account = accounts.get(5)?;
        let asset = accounts.get(6)?;
        let sell_state = accounts.get(7)?;
        let collection = accounts.get(8)?;
        let system_program = accounts.get(9)?;
        let asset_program = accounts.get(10)?;

        Some(SolMplCoreFulfillSellInstructionAccounts {
            payer: payer.pubkey,
            owner: owner.pubkey,
            cosigner: cosigner.pubkey,
            referral: referral.pubkey,
            pool: pool.pubkey,
            buyside_sol_escrow_account: buyside_sol_escrow_account.pubkey,
            asset: asset.pubkey,
            sell_state: sell_state.pubkey,
            collection: collection.pubkey,
            system_program: system_program.pubkey,
            asset_program: asset_program.pubkey,
        })
    }
}