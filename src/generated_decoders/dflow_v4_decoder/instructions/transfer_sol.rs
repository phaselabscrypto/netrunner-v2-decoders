use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4e0aecf76d75154c")]
pub struct TransferSol {
    pub lamports: u64,
}

pub struct TransferSolInstructionAccounts {
    pub from: solana_sdk::pubkey::Pubkey,
    pub to: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferSol {
    type ArrangedAccounts = TransferSolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let from = accounts.get(0)?;
        let to = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(TransferSolInstructionAccounts {
            from: from.pubkey,
            to: to.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
