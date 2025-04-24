use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8e89a76b2f27f07c")]
pub struct InitializeCandyMachine {
    pub bump: u8,
    pub data: CandyMachineData,
}

pub struct InitializeCandyMachineInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub config: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeCandyMachine {
    type ArrangedAccounts = InitializeCandyMachineInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let wallet = accounts.get(1)?;
        let config = accounts.get(2)?;
        let authority = accounts.get(3)?;
        let payer = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let rent = accounts.get(6)?;

        Some(InitializeCandyMachineInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            wallet: wallet.pubkey,
            config: config.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
