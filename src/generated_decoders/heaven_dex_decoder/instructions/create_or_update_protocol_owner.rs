use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xaa7c802830698b94")]
pub struct CreateOrUpdateProtocolOwner {}

pub struct CreateOrUpdateProtocolOwnerInstructionAccounts {
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub current_owner: solana_sdk::pubkey::Pubkey,
    pub new_owner: solana_sdk::pubkey::Pubkey,
    pub protocol_owner_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOrUpdateProtocolOwner {
    type ArrangedAccounts = CreateOrUpdateProtocolOwnerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let system_program = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let current_owner = accounts.get(2)?;
        let new_owner = accounts.get(3)?;
        let protocol_owner_state = accounts.get(4)?;

        Some(CreateOrUpdateProtocolOwnerInstructionAccounts {
            system_program: system_program.pubkey,
            payer: payer.pubkey,
            current_owner: current_owner.pubkey,
            new_owner: new_owner.pubkey,
            protocol_owner_state: protocol_owner_state.pubkey,
        })
    }
}
