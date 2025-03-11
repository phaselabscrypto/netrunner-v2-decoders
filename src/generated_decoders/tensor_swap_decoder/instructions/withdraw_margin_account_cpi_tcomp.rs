

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc99ca31bf30e24ed")]
pub struct WithdrawMarginAccountCpiTcomp{
    pub bump: u8,
    pub bid_id: solana_sdk::pubkey::Pubkey,
    pub lamports: u64,
}

pub struct WithdrawMarginAccountCpiTcompInstructionAccounts {
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub bid_state: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawMarginAccountCpiTcomp {
    type ArrangedAccounts = WithdrawMarginAccountCpiTcompInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let margin_account = accounts.get(0)?;
        let bid_state = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let destination = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(WithdrawMarginAccountCpiTcompInstructionAccounts {
            margin_account: margin_account.pubkey,
            bid_state: bid_state.pubkey,
            owner: owner.pubkey,
            destination: destination.pubkey,
            system_program: system_program.pubkey,
        })
    }
}