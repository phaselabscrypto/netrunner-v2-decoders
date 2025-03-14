

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x49150440a1d6f84d")]
pub struct DepositNftCore{
}

pub struct DepositNftCoreInstructionAccounts {
    pub transfer: solana_sdk::pubkey::Pubkey,
    pub core: solana_sdk::pubkey::Pubkey,
    pub nft_receipt: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositNftCore {
    type ArrangedAccounts = DepositNftCoreInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let transfer = accounts.get(0)?;
        let core = accounts.get(1)?;
        let nft_receipt = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(DepositNftCoreInstructionAccounts {
            transfer: transfer.pubkey,
            core: core.pubkey,
            nft_receipt: nft_receipt.pubkey,
            system_program: system_program.pubkey,
        })
    }
}