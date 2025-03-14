

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x69d729efa6cf0167")]
pub struct CloseMarginAccount{
}

pub struct CloseMarginAccountInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseMarginAccount {
    type ArrangedAccounts = CloseMarginAccountInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let margin_account = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(CloseMarginAccountInstructionAccounts {
            tswap: tswap.pubkey,
            margin_account: margin_account.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
        })
    }
}