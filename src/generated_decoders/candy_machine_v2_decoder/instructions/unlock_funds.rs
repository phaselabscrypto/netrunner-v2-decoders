use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xaf7710f58d37ff2b")]
pub struct UnlockFunds {}

pub struct UnlockFundsInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub freeze_pda: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UnlockFunds {
    type ArrangedAccounts = UnlockFundsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let wallet = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let freeze_pda = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(UnlockFundsInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            wallet: wallet.pubkey,
            authority: authority.pubkey,
            freeze_pda: freeze_pda.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
