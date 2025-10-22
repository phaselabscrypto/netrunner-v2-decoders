use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3f5794216d230868")]
pub struct CreateOperationAccount {}

pub struct CreateOperationAccountInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub admin_group: solana_sdk::pubkey::Pubkey,
    pub operation_state: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOperationAccount {
    type ArrangedAccounts = CreateOperationAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let admin_group = accounts.get(1)?;
        let operation_state = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(CreateOperationAccountInstructionAccounts {
            owner: owner.pubkey,
            admin_group: admin_group.pubkey,
            operation_state: operation_state.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
