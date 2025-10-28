use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xadd051080d13ca96")]
pub struct ClaimLockedAndStake {}

pub struct ClaimLockedAndStakeInstructionAccounts {
    pub distributor: solana_sdk::pubkey::Pubkey,
    pub claim_status: solana_sdk::pubkey::Pubkey,
    pub from: solana_sdk::pubkey::Pubkey,
    pub claimant: solana_sdk::pubkey::Pubkey,
    pub operator: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub voter_program: solana_sdk::pubkey::Pubkey,
    pub locker: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_tokens: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimLockedAndStake {
    type ArrangedAccounts = ClaimLockedAndStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [distributor, claim_status, from, claimant, operator, token_program, voter_program, locker, escrow, escrow_tokens, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ClaimLockedAndStakeInstructionAccounts {
            distributor: distributor.pubkey,
            claim_status: claim_status.pubkey,
            from: from.pubkey,
            claimant: claimant.pubkey,
            operator: operator.pubkey,
            token_program: token_program.pubkey,
            voter_program: voter_program.pubkey,
            locker: locker.pubkey,
            escrow: escrow.pubkey,
            escrow_tokens: escrow_tokens.pubkey,
        })
    }
}
