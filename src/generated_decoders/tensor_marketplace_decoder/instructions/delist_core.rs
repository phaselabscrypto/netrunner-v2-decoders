

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x3818e702e3130e44")]
pub struct DelistCore{
}

pub struct DelistCoreInstructionAccounts {
    pub asset: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub list_state: solana_sdk::pubkey::Pubkey,
    pub mpl_core_program: solana_sdk::pubkey::Pubkey,
    pub tcomp_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent_dest: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DelistCore {
    type ArrangedAccounts = DelistCoreInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let asset = accounts.get(0)?;
        let collection = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let list_state = accounts.get(3)?;
        let mpl_core_program = accounts.get(4)?;
        let tcomp_program = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let rent_dest = accounts.get(7)?;

        Some(DelistCoreInstructionAccounts {
            asset: asset.pubkey,
            collection: collection.pubkey,
            owner: owner.pubkey,
            list_state: list_state.pubkey,
            mpl_core_program: mpl_core_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            system_program: system_program.pubkey,
            rent_dest: rent_dest.pubkey,
        })
    }
}