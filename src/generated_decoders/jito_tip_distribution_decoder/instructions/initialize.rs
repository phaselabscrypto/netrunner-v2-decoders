use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub expired_funds_account: solana_sdk::pubkey::Pubkey,
    pub num_epochs_valid: u64,
    pub max_validator_commission_bps: u16,
    pub bump: u8,
}

pub struct InitializeInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub initializer: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let system_program = accounts.get(1)?;
        let initializer = accounts.get(2)?;

        Some(InitializeInstructionAccounts {
            config: config.pubkey,
            system_program: system_program.pubkey,
            initializer: initializer.pubkey,
        })
    }
}
