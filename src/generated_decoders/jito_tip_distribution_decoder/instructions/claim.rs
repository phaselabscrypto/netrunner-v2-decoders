use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3ec6d6c1d59f6cd2")]
pub struct Claim {
    pub bump: u8,
    pub amount: u64,
    pub proof: Vec<[u8; 32]>,
}

pub struct ClaimInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub tip_distribution_account: solana_sdk::pubkey::Pubkey,
    pub claim_status: solana_sdk::pubkey::Pubkey,
    pub claimant: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Claim {
    type ArrangedAccounts = ClaimInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let tip_distribution_account = accounts.get(1)?;
        let claim_status = accounts.get(2)?;
        let claimant = accounts.get(3)?;
        let payer = accounts.get(4)?;
        let system_program = accounts.get(5)?;

        Some(ClaimInstructionAccounts {
            config: config.pubkey,
            tip_distribution_account: tip_distribution_account.pubkey,
            claim_status: claim_status.pubkey,
            claimant: claimant.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
