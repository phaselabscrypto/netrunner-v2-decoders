

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x8bab5e92bf5b9032")]
pub struct CloseEscrow{
}

pub struct CloseEscrowInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub rent_receiver: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseEscrow {
    type ArrangedAccounts = CloseEscrowInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let escrow = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let rent_receiver = accounts.get(3)?;
        let event_authority = accounts.get(4)?;
        let program = accounts.get(5)?;

        Some(CloseEscrowInstructionAccounts {
            vault: vault.pubkey,
            escrow: escrow.pubkey,
            owner: owner.pubkey,
            rent_receiver: rent_receiver.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}