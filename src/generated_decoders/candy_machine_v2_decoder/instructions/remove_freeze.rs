

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x01d450a8813c2efb")]
pub struct RemoveFreeze{
}

pub struct RemoveFreezeInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub freeze_pda: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveFreeze {
    type ArrangedAccounts = RemoveFreezeInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let freeze_pda = accounts.get(2)?;

        Some(RemoveFreezeInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority: authority.pubkey,
            freeze_pda: freeze_pda.pubkey,
        })
    }
}