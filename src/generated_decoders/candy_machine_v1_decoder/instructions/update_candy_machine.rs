use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf3fb7c9cd3d376ef")]
pub struct UpdateCandyMachine {
    pub price: Option<u64>,
    pub go_live_date: Option<i64>,
}

pub struct UpdateCandyMachineInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateCandyMachine {
    type ArrangedAccounts = UpdateCandyMachineInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority = accounts.get(1)?;

        Some(UpdateCandyMachineInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority: authority.pubkey,
        })
    }
}
