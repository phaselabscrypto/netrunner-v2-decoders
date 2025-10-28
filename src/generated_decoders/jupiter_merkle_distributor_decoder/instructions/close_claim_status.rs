use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa3d6bfa5f5bc11b9")]
pub struct CloseClaimStatus {}

pub struct CloseClaimStatusInstructionAccounts {
    pub claim_status: solana_sdk::pubkey::Pubkey,
    pub claimant: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseClaimStatus {
    type ArrangedAccounts = CloseClaimStatusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [claim_status, claimant, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CloseClaimStatusInstructionAccounts {
            claim_status: claim_status.pubkey,
            claimant: claimant.pubkey,
            admin: admin.pubkey,
        })
    }
}
