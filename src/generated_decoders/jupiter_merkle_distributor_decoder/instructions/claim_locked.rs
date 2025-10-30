use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x22ceb5170bcf935a")]
pub struct ClaimLocked {}

pub struct ClaimLockedInstructionAccounts {
    pub distributor: solana_sdk::pubkey::Pubkey,
    pub claim_status: solana_sdk::pubkey::Pubkey,
    pub from: solana_sdk::pubkey::Pubkey,
    pub to: solana_sdk::pubkey::Pubkey,
    pub claimant: solana_sdk::pubkey::Pubkey,
    pub operator: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimLocked {
    type ArrangedAccounts = ClaimLockedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [distributor, claim_status, from, to, claimant, operator, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ClaimLockedInstructionAccounts {
            distributor: distributor.pubkey,
            claim_status: claim_status.pubkey,
            from: from.pubkey,
            to: to.pubkey,
            claimant: claimant.pubkey,
            operator: operator.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
