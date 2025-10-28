use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbe9aa399a87328ad")]
pub struct MergePartialUnstaking {}

pub struct MergePartialUnstakingInstructionAccounts {
    pub locker: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub partial_unstake: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MergePartialUnstaking {
    type ArrangedAccounts = MergePartialUnstakingInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [locker, escrow, partial_unstake, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(MergePartialUnstakingInstructionAccounts {
            locker: locker.pubkey,
            escrow: escrow.pubkey,
            partial_unstake: partial_unstake.pubkey,
            owner: owner.pubkey,
        })
    }
}
