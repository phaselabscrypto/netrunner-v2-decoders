

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xad4ca77d76470199")]
pub struct ListCore{
    pub amount: u64,
    pub expire_in_sec: Option<u64>,
    pub currency: Option<solana_sdk::pubkey::Pubkey>,
    pub private_taker: Option<solana_sdk::pubkey::Pubkey>,
    pub maker_broker: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct ListCoreInstructionAccounts {
    pub asset: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub list_state: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub mpl_core_program: solana_sdk::pubkey::Pubkey,
    pub tcomp_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ListCore {
    type ArrangedAccounts = ListCoreInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let asset = accounts.get(0)?;
        let collection = accounts.get(1)?;
        let list_state = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let mpl_core_program = accounts.get(4)?;
        let tcomp_program = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let payer = accounts.get(7)?;

        Some(ListCoreInstructionAccounts {
            asset: asset.pubkey,
            collection: collection.pubkey,
            list_state: list_state.pubkey,
            owner: owner.pubkey,
            mpl_core_program: mpl_core_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            system_program: system_program.pubkey,
            payer: payer.pubkey,
        })
    }
}