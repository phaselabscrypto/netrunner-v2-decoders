use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6f5c8e4f21ea521b")]
pub struct Clawback {}

pub struct ClawbackInstructionAccounts {
    pub distributor: solana_sdk::pubkey::Pubkey,
    pub from: solana_sdk::pubkey::Pubkey,
    pub clawback_receiver: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Clawback {
    type ArrangedAccounts = ClawbackInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [distributor, from, clawback_receiver, token_program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(ClawbackInstructionAccounts {
            distributor: distributor.pubkey,
            from: from.pubkey,
            clawback_receiver: clawback_receiver.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
