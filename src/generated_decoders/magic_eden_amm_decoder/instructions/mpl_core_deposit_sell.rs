
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xee44eefec6da5a72")]
pub struct MplCoreDepositSell{
    pub args: MplCoreDepositSellArgs,
}

pub struct MplCoreDepositSellInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub asset: solana_sdk::pubkey::Pubkey,
    pub sell_state: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub asset_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MplCoreDepositSell {
    type ArrangedAccounts = MplCoreDepositSellInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let cosigner = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let asset = accounts.get(3)?;
        let sell_state = accounts.get(4)?;
        let collection = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let asset_program = accounts.get(7)?;

        Some(MplCoreDepositSellInstructionAccounts {
            owner: owner.pubkey,
            cosigner: cosigner.pubkey,
            pool: pool.pubkey,
            asset: asset.pubkey,
            sell_state: sell_state.pubkey,
            collection: collection.pubkey,
            system_program: system_program.pubkey,
            asset_program: asset_program.pubkey,
        })
    }
}