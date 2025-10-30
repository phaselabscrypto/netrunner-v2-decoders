use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x326ff27633fa8dbb")]
pub struct NewClaimAndStake {
    pub amount_unlocked: u64,
    pub amount_locked: u64,
    pub proof: Vec<[u8; 32]>,
}

pub struct NewClaimAndStakeInstructionAccounts {
    pub distributor: solana_sdk::pubkey::Pubkey,
    pub claim_status: solana_sdk::pubkey::Pubkey,
    pub from: solana_sdk::pubkey::Pubkey,
    pub claimant: solana_sdk::pubkey::Pubkey,
    pub operator: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub voter_program: solana_sdk::pubkey::Pubkey,
    pub locker: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_tokens: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for NewClaimAndStake {
    type ArrangedAccounts = NewClaimAndStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [distributor, claim_status, from, claimant, operator, token_program, system_program, voter_program, locker, escrow, escrow_tokens, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(NewClaimAndStakeInstructionAccounts {
            distributor: distributor.pubkey,
            claim_status: claim_status.pubkey,
            from: from.pubkey,
            claimant: claimant.pubkey,
            operator: operator.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            voter_program: voter_program.pubkey,
            locker: locker.pubkey,
            escrow: escrow.pubkey,
            escrow_tokens: escrow_tokens.pubkey,
        })
    }
}
