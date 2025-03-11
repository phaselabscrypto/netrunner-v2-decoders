

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x7083ef74bb957291")]
pub struct WithdrawNftCore{
}

pub struct WithdrawNftCoreInstructionAccounts {
    pub transfer: solana_sdk::pubkey::Pubkey,
    pub core: solana_sdk::pubkey::Pubkey,
    pub nft_receipt: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawNftCore {
    type ArrangedAccounts = WithdrawNftCoreInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let transfer = accounts.get(0)?;
        let core = accounts.get(1)?;
        let nft_receipt = accounts.get(2)?;

        Some(WithdrawNftCoreInstructionAccounts {
            transfer: transfer.pubkey,
            core: core.pubkey,
            nft_receipt: nft_receipt.pubkey,
        })
    }
}