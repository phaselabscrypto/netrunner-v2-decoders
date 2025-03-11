

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xcfeba6ffa3a2952c")]
pub struct WithdrawMarginAccountCpiTlock{
    pub bump: u8,
    pub order_id: [u8; 32],
    pub lamports: u64,
}

pub struct WithdrawMarginAccountCpiTlockInstructionAccounts {
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub order_state: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawMarginAccountCpiTlock {
    type ArrangedAccounts = WithdrawMarginAccountCpiTlockInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let margin_account = accounts.get(0)?;
        let order_state = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let destination = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(WithdrawMarginAccountCpiTlockInstructionAccounts {
            margin_account: margin_account.pubkey,
            order_state: order_state.pubkey,
            owner: owner.pubkey,
            destination: destination.pubkey,
            system_program: system_program.pubkey,
        })
    }
}