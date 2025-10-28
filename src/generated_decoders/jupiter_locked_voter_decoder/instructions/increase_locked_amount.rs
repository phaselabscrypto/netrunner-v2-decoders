use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x05a87635482ecb92")]
pub struct IncreaseLockedAmount {
    pub amount: u64,
}

pub struct IncreaseLockedAmountInstructionAccounts {
    pub locker: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_tokens: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub source_tokens: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IncreaseLockedAmount {
    type ArrangedAccounts = IncreaseLockedAmountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [locker, escrow, escrow_tokens, payer, source_tokens, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(IncreaseLockedAmountInstructionAccounts {
            locker: locker.pubkey,
            escrow: escrow.pubkey,
            escrow_tokens: escrow_tokens.pubkey,
            payer: payer.pubkey,
            source_tokens: source_tokens.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
