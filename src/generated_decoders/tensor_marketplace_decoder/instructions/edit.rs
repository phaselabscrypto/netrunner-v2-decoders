

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0fb72156571c9791")]
pub struct Edit{
    pub amount: u64,
    pub expire_in_sec: Option<u64>,
    pub currency: Option<solana_sdk::pubkey::Pubkey>,
    pub private_taker: Option<solana_sdk::pubkey::Pubkey>,
    pub maker_broker: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct EditInstructionAccounts {
    pub list_state: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub tcomp_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Edit {
    type ArrangedAccounts = EditInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let list_state = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let tcomp_program = accounts.get(2)?;

        Some(EditInstructionAccounts {
            list_state: list_state.pubkey,
            owner: owner.pubkey,
            tcomp_program: tcomp_program.pubkey,
        })
    }
}