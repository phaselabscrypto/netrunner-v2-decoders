use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xca506dd082901ae9")]
pub struct SetFreeze {
    pub freeze_time: i64,
}

pub struct SetFreezeInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub freeze_pda: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetFreeze {
    type ArrangedAccounts = SetFreezeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let freeze_pda = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(SetFreezeInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority: authority.pubkey,
            freeze_pda: freeze_pda.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
