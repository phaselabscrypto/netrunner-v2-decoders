use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x157eb014556f351f")]
pub struct CreateOrUpdateProtocolFeeAdmin {}

pub struct CreateOrUpdateProtocolFeeAdminInstructionAccounts {
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub current_owner: solana_sdk::pubkey::Pubkey,
    pub protocol_owner_state: solana_sdk::pubkey::Pubkey,
    pub new_admin: solana_sdk::pubkey::Pubkey,
    pub protocol_fee_admin_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOrUpdateProtocolFeeAdmin {
    type ArrangedAccounts = CreateOrUpdateProtocolFeeAdminInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let system_program = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let current_owner = accounts.get(2)?;
        let protocol_owner_state = accounts.get(3)?;
        let new_admin = accounts.get(4)?;
        let protocol_fee_admin_state = accounts.get(5)?;

        Some(CreateOrUpdateProtocolFeeAdminInstructionAccounts {
            system_program: system_program.pubkey,
            payer: payer.pubkey,
            current_owner: current_owner.pubkey,
            protocol_owner_state: protocol_owner_state.pubkey,
            new_admin: new_admin.pubkey,
            protocol_fee_admin_state: protocol_fee_admin_state.pubkey,
        })
    }
}
