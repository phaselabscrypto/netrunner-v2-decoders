

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1a")]
pub struct ClaimWithPayer{
    pub proof: Vec<[u8; 32]>,
    pub amount: u64,
    pub bump: u8,
}

pub struct ClaimWithPayerInstructionAccounts {
    pub account_payer: solana_sdk::pubkey::Pubkey,
    pub config: solana_sdk::pubkey::Pubkey,
    pub ncn: solana_sdk::pubkey::Pubkey,
    pub tip_distribution_config: solana_sdk::pubkey::Pubkey,
    pub tip_distribution_account: solana_sdk::pubkey::Pubkey,
    pub claim_status: solana_sdk::pubkey::Pubkey,
    pub claimant: solana_sdk::pubkey::Pubkey,
    pub tip_distribution_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimWithPayer {
    type ArrangedAccounts = ClaimWithPayerInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let account_payer = accounts.get(0)?;
        let config = accounts.get(1)?;
        let ncn = accounts.get(2)?;
        let tip_distribution_config = accounts.get(3)?;
        let tip_distribution_account = accounts.get(4)?;
        let claim_status = accounts.get(5)?;
        let claimant = accounts.get(6)?;
        let tip_distribution_program = accounts.get(7)?;
        let system_program = accounts.get(8)?;

        Some(ClaimWithPayerInstructionAccounts {
            account_payer: account_payer.pubkey,
            config: config.pubkey,
            ncn: ncn.pubkey,
            tip_distribution_config: tip_distribution_config.pubkey,
            tip_distribution_account: tip_distribution_account.pubkey,
            claim_status: claim_status.pubkey,
            claimant: claimant.pubkey,
            tip_distribution_program: tip_distribution_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}