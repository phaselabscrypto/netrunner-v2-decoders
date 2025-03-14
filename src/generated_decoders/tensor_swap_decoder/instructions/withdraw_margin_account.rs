

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x364996d0cf051211")]
pub struct WithdrawMarginAccount{
    pub lamports: u64,
}

pub struct WithdrawMarginAccountInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawMarginAccount {
    type ArrangedAccounts = WithdrawMarginAccountInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let margin_account = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(WithdrawMarginAccountInstructionAccounts {
            tswap: tswap.pubkey,
            margin_account: margin_account.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
        })
    }
}