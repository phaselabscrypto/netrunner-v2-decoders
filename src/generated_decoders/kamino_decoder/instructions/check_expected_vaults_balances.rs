

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4b97bb7d32040b47")]
pub struct CheckExpectedVaultsBalances{
    pub token_a_ata_balance: u64,
    pub token_b_ata_balance: u64,
}

pub struct CheckExpectedVaultsBalancesInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub token_a_ata: solana_sdk::pubkey::Pubkey,
    pub token_b_ata: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CheckExpectedVaultsBalances {
    type ArrangedAccounts = CheckExpectedVaultsBalancesInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let user = accounts.get(0)?;
        let token_a_ata = accounts.get(1)?;
        let token_b_ata = accounts.get(2)?;

        Some(CheckExpectedVaultsBalancesInstructionAccounts {
            user: user.pubkey,
            token_a_ata: token_a_ata.pubkey,
            token_b_ata: token_b_ata.pubkey,
        })
    }
}