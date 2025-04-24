use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf50ec0d3632aaabb")]
pub struct CloseProgramAccount {}

pub struct CloseProgramAccountInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub program_data: solana_sdk::pubkey::Pubkey,
    pub closing_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseProgramAccount {
    type ArrangedAccounts = CloseProgramAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let program = accounts.get(1)?;
        let program_data = accounts.get(2)?;
        let closing_account = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(CloseProgramAccountInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            program: program.pubkey,
            program_data: program_data.pubkey,
            closing_account: closing_account.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
