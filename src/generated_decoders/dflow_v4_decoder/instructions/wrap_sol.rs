use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2f3e9bac83cd25c9")]
pub struct WrapSol {
    pub lamports: u64,
}

pub struct WrapSolInstructionAccounts {
    pub from: solana_sdk::pubkey::Pubkey,
    pub wrapped_sol_associated_token_account: solana_sdk::pubkey::Pubkey,
    pub native_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WrapSol {
    type ArrangedAccounts = WrapSolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let from = accounts.get(0)?;
        let wrapped_sol_associated_token_account = accounts.get(1)?;
        let native_mint = accounts.get(2)?;
        let token_program = accounts.get(3)?;
        let associated_token_program = accounts.get(4)?;
        let system_program = accounts.get(5)?;

        Some(WrapSolInstructionAccounts {
            from: from.pubkey,
            wrapped_sol_associated_token_account: wrapped_sol_associated_token_account.pubkey,
            native_mint: native_mint.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
