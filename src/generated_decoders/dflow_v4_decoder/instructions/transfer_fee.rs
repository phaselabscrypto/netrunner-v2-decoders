use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x81a4c415b130b4a2")]
pub struct TransferFee {
    pub amount: u64,
}

pub struct TransferFeeInstructionAccounts {
    pub from: solana_sdk::pubkey::Pubkey,
    pub to: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferFee {
    type ArrangedAccounts = TransferFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let from = accounts.get(0)?;
        let to = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let token_program = accounts.get(3)?;
        let event_authority = accounts.get(4)?;
        let program = accounts.get(5)?;

        Some(TransferFeeInstructionAccounts {
            from: from.pubkey,
            to: to.pubkey,
            authority: authority.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
