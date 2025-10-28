use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5abacbea46b9bf15")]
pub struct ActivateProposal {}

pub struct ActivateProposalInstructionAccounts {
    pub locker: solana_sdk::pubkey::Pubkey,
    pub governor: solana_sdk::pubkey::Pubkey,
    pub proposal: solana_sdk::pubkey::Pubkey,
    pub govern_program: solana_sdk::pubkey::Pubkey,
    pub smart_wallet: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ActivateProposal {
    type ArrangedAccounts = ActivateProposalInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [locker, governor, proposal, govern_program, smart_wallet, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(ActivateProposalInstructionAccounts {
            locker: locker.pubkey,
            governor: governor.pubkey,
            proposal: proposal.pubkey,
            govern_program: govern_program.pubkey,
            smart_wallet: smart_wallet.pubkey,
        })
    }
}
