use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize {}

pub struct InitializeInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub atlas: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let atlas = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(InitializeInstructionAccounts {
            payer: payer.pubkey,
            atlas: atlas.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
