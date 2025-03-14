

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xb712469c946da122")]
pub struct Withdraw{
    pub escrow_payment_bump: u8,
    pub amount: u64,
}

pub struct WithdrawInstructionAccounts {
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub notary: solana_sdk::pubkey::Pubkey,
    pub escrow_payment_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let wallet = accounts.get(0)?;
        let notary = accounts.get(1)?;
        let escrow_payment_account = accounts.get(2)?;
        let authority = accounts.get(3)?;
        let auction_house = accounts.get(4)?;
        let system_program = accounts.get(5)?;

        Some(WithdrawInstructionAccounts {
            wallet: wallet.pubkey,
            notary: notary.pubkey,
            escrow_payment_account: escrow_payment_account.pubkey,
            authority: authority.pubkey,
            auction_house: auction_house.pubkey,
            system_program: system_program.pubkey,
        })
    }
}