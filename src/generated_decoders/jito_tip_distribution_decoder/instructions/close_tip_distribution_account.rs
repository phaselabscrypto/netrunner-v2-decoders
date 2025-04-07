

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x2f88d0be7df34ae3")]
pub struct CloseTipDistributionAccount{
    pub epoch: u64,
}

pub struct CloseTipDistributionAccountInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub expired_funds_account: solana_sdk::pubkey::Pubkey,
    pub tip_distribution_account: solana_sdk::pubkey::Pubkey,
    pub validator_vote_account: solana_sdk::pubkey::Pubkey,
    pub signer: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseTipDistributionAccount {
    type ArrangedAccounts = CloseTipDistributionAccountInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let expired_funds_account = accounts.get(1)?;
        let tip_distribution_account = accounts.get(2)?;
        let validator_vote_account = accounts.get(3)?;
        let signer = accounts.get(4)?;

        Some(CloseTipDistributionAccountInstructionAccounts {
            config: config.pubkey,
            expired_funds_account: expired_funds_account.pubkey,
            tip_distribution_account: tip_distribution_account.pubkey,
            validator_vote_account: validator_vote_account.pubkey,
            signer: signer.pubkey,
        })
    }
}