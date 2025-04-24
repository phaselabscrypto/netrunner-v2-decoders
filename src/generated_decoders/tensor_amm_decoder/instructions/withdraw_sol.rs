use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x91834a8841892a26")]
pub struct WithdrawSol {
    pub lamports: u64,
}

pub struct WithdrawSolInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawSol {
    type ArrangedAccounts = WithdrawSolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(WithdrawSolInstructionAccounts {
            owner: owner.pubkey,
            pool: pool.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
