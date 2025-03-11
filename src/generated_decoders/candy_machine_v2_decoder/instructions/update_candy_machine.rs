
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf3fb7c9cd3d376ef")]
pub struct UpdateCandyMachine{
    pub data: CandyMachineData,
}

pub struct UpdateCandyMachineInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub wallet: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateCandyMachine {
    type ArrangedAccounts = UpdateCandyMachineInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let wallet = accounts.get(2)?;

        Some(UpdateCandyMachineInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority: authority.pubkey,
            wallet: wallet.pubkey,
        })
    }
}