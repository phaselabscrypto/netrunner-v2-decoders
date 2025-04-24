use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x407c0639979b1ac3")]
pub struct TakeFlashLoan {
    pub lamports: u64,
}

pub struct TakeFlashLoanInstructionAccounts {
    pub receiver: solana_sdk::pubkey::Pubkey,
    pub pool_account: solana_sdk::pubkey::Pubkey,
    pub pool_sol_reserves: solana_sdk::pubkey::Pubkey,
    pub flash_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub instructions: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TakeFlashLoan {
    type ArrangedAccounts = TakeFlashLoanInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let receiver = accounts.get(0)?;
        let pool_account = accounts.get(1)?;
        let pool_sol_reserves = accounts.get(2)?;
        let flash_account = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let instructions = accounts.get(5)?;

        Some(TakeFlashLoanInstructionAccounts {
            receiver: receiver.pubkey,
            pool_account: pool_account.pubkey,
            pool_sol_reserves: pool_sol_reserves.pubkey,
            flash_account: flash_account.pubkey,
            system_program: system_program.pubkey,
            instructions: instructions.pubkey,
        })
    }
}
