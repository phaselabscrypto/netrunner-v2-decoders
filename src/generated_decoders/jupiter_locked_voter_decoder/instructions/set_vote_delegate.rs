use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2eecf1f3fb6c9c0c")]
pub struct SetVoteDelegate {
    pub new_delegate: solana_sdk::pubkey::Pubkey,
}

pub struct SetVoteDelegateInstructionAccounts {
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_owner: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetVoteDelegate {
    type ArrangedAccounts = SetVoteDelegateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [escrow, escrow_owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetVoteDelegateInstructionAccounts {
            escrow: escrow.pubkey,
            escrow_owner: escrow_owner.pubkey,
        })
    }
}
