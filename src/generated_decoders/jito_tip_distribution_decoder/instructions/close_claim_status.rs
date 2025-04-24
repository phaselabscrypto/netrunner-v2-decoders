use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa3d6bfa5f5bc11b9")]
pub struct CloseClaimStatus {}

pub struct CloseClaimStatusInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub claim_status: solana_sdk::pubkey::Pubkey,
    pub claim_status_payer: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseClaimStatus {
    type ArrangedAccounts = CloseClaimStatusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let claim_status = accounts.get(1)?;
        let claim_status_payer = accounts.get(2)?;

        Some(CloseClaimStatusInstructionAccounts {
            config: config.pubkey,
            claim_status: claim_status.pubkey,
            claim_status_payer: claim_status_payer.pubkey,
        })
    }
}
