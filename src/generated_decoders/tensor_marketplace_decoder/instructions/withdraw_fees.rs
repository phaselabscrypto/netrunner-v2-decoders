

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc6d4ab6d90d7ae59")]
pub struct WithdrawFees{
    pub amount: u64,
}

pub struct WithdrawFeesInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub tcomp: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawFees {
    type ArrangedAccounts = WithdrawFeesInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let tcomp = accounts.get(1)?;
        let cosigner = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let destination = accounts.get(4)?;
        let system_program = accounts.get(5)?;

        Some(WithdrawFeesInstructionAccounts {
            tswap: tswap.pubkey,
            tcomp: tcomp.pubkey,
            cosigner: cosigner.pubkey,
            owner: owner.pubkey,
            destination: destination.pubkey,
            system_program: system_program.pubkey,
        })
    }
}