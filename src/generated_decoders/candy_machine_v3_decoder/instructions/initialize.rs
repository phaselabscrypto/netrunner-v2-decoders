use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize {
    pub data: Vec<u8>,
}

pub struct InitializeInstructionAccounts {
    pub candy_guard: solana_sdk::pubkey::Pubkey,
    pub base: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let candy_guard = accounts.get(0)?;
        let base = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(InitializeInstructionAccounts {
            candy_guard: candy_guard.pubkey,
            base: base.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
