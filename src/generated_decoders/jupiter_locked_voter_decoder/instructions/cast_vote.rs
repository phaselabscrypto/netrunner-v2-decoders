use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x14d40fbd45b44597")]
pub struct CastVote {
    pub side: u8,
}

pub struct CastVoteInstructionAccounts {
    pub locker: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub vote_delegate: solana_sdk::pubkey::Pubkey,
    pub proposal: solana_sdk::pubkey::Pubkey,
    pub vote: solana_sdk::pubkey::Pubkey,
    pub governor: solana_sdk::pubkey::Pubkey,
    pub govern_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CastVote {
    type ArrangedAccounts = CastVoteInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [locker, escrow, vote_delegate, proposal, vote, governor, govern_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CastVoteInstructionAccounts {
            locker: locker.pubkey,
            escrow: escrow.pubkey,
            vote_delegate: vote_delegate.pubkey,
            proposal: proposal.pubkey,
            vote: vote.pubkey,
            governor: governor.pubkey,
            govern_program: govern_program.pubkey,
        })
    }
}
