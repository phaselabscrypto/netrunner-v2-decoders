

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x78bf19b66f31b337")]
pub struct InitializeTipDistributionAccount{
    pub merkle_root_upload_authority: solana_sdk::pubkey::Pubkey,
    pub validator_commission_bps: u16,
    pub bump: u8,
}

pub struct InitializeTipDistributionAccountInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub tip_distribution_account: solana_sdk::pubkey::Pubkey,
    pub validator_vote_account: solana_sdk::pubkey::Pubkey,
    pub signer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeTipDistributionAccount {
    type ArrangedAccounts = InitializeTipDistributionAccountInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let tip_distribution_account = accounts.get(1)?;
        let validator_vote_account = accounts.get(2)?;
        let signer = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(InitializeTipDistributionAccountInstructionAccounts {
            config: config.pubkey,
            tip_distribution_account: tip_distribution_account.pubkey,
            validator_vote_account: validator_vote_account.pubkey,
            signer: signer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}