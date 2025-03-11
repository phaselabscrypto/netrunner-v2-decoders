

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xba1ac786dcb12048")]
pub struct WithdrawMarginAccountCpi{
    pub bump: u8,
    pub lamports: u64,
}

pub struct WithdrawMarginAccountCpiInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub bid_state: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawMarginAccountCpi {
    type ArrangedAccounts = WithdrawMarginAccountCpiInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let margin_account = accounts.get(1)?;
        let bid_state = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let nft_mint = accounts.get(4)?;
        let destination = accounts.get(5)?;
        let system_program = accounts.get(6)?;

        Some(WithdrawMarginAccountCpiInstructionAccounts {
            tswap: tswap.pubkey,
            margin_account: margin_account.pubkey,
            bid_state: bid_state.pubkey,
            owner: owner.pubkey,
            nft_mint: nft_mint.pubkey,
            destination: destination.pubkey,
            system_program: system_program.pubkey,
        })
    }
}