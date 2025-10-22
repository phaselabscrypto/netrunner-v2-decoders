use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x63280e692d6bacc9")]
pub struct UnwrapSol {}

pub struct UnwrapSolInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub wrapped_sol_associated_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UnwrapSol {
    type ArrangedAccounts = UnwrapSolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let wrapped_sol_associated_token_account = accounts.get(1)?;
        let token_program = accounts.get(2)?;

        Some(UnwrapSolInstructionAccounts {
            owner: owner.pubkey,
            wrapped_sol_associated_token_account: wrapped_sol_associated_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
