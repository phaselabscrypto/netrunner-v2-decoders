

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x59ab4e504abc3f3a")]
pub struct CloseExpiredListingCore{
}

pub struct CloseExpiredListingCoreInstructionAccounts {
    pub list_state: solana_sdk::pubkey::Pubkey,
    pub asset: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub mpl_core_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub tcomp_program: solana_sdk::pubkey::Pubkey,
    pub rent_dest: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseExpiredListingCore {
    type ArrangedAccounts = CloseExpiredListingCoreInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let list_state = accounts.get(0)?;
        let asset = accounts.get(1)?;
        let collection = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let mpl_core_program = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let tcomp_program = accounts.get(6)?;
        let rent_dest = accounts.get(7)?;

        Some(CloseExpiredListingCoreInstructionAccounts {
            list_state: list_state.pubkey,
            asset: asset.pubkey,
            collection: collection.pubkey,
            owner: owner.pubkey,
            mpl_core_program: mpl_core_program.pubkey,
            system_program: system_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            rent_dest: rent_dest.pubkey,
        })
    }
}