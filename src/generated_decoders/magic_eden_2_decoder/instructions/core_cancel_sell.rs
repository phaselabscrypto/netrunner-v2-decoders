
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x424427ed09f77afd")]
pub struct CoreCancelSell{
    pub args: CoreCancelSellArgs,
}

pub struct CoreCancelSellInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub notary: solana_sdk::pubkey::Pubkey,
    pub program_as_signer: solana_sdk::pubkey::Pubkey,
    pub asset: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub seller_trade_state: solana_sdk::pubkey::Pubkey,
    pub asset_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CoreCancelSell {
    type ArrangedAccounts = CoreCancelSellInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let wallet = accounts.get(1)?;
        let notary = accounts.get(2)?;
        let program_as_signer = accounts.get(3)?;
        let asset = accounts.get(4)?;
        let auction_house = accounts.get(5)?;
        let seller_trade_state = accounts.get(6)?;
        let asset_program = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let collection = accounts.get(9)?;

        Some(CoreCancelSellInstructionAccounts {
            payer: payer.pubkey,
            wallet: wallet.pubkey,
            notary: notary.pubkey,
            program_as_signer: program_as_signer.pubkey,
            asset: asset.pubkey,
            auction_house: auction_house.pubkey,
            seller_trade_state: seller_trade_state.pubkey,
            asset_program: asset_program.pubkey,
            system_program: system_program.pubkey,
            collection: collection.pubkey,
        })
    }
}