

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x53146943f84468be")]
pub struct CloseExpiredBid{
}

pub struct CloseExpiredBidInstructionAccounts {
    pub bid_state: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub tcomp_program: solana_sdk::pubkey::Pubkey,
    pub rent_dest: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseExpiredBid {
    type ArrangedAccounts = CloseExpiredBidInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let bid_state = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let system_program = accounts.get(2)?;
        let tcomp_program = accounts.get(3)?;
        let rent_dest = accounts.get(4)?;

        Some(CloseExpiredBidInstructionAccounts {
            bid_state: bid_state.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            rent_dest: rent_dest.pubkey,
        })
    }
}