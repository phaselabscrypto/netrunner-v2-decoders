use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x04acc4d578321e89")]
pub struct CreateOrUpdateProtocolStakingAdmin {}

pub struct CreateOrUpdateProtocolStakingAdminInstructionAccounts {
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub current_owner: solana_sdk::pubkey::Pubkey,
    pub protocol_owner_state: solana_sdk::pubkey::Pubkey,
    pub new_admin: solana_sdk::pubkey::Pubkey,
    pub protocol_staking_admin_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOrUpdateProtocolStakingAdmin {
    type ArrangedAccounts = CreateOrUpdateProtocolStakingAdminInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let system_program = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let current_owner = accounts.get(2)?;
        let protocol_owner_state = accounts.get(3)?;
        let new_admin = accounts.get(4)?;
        let protocol_staking_admin_state = accounts.get(5)?;

        Some(CreateOrUpdateProtocolStakingAdminInstructionAccounts {
            system_program: system_program.pubkey,
            payer: payer.pubkey,
            current_owner: current_owner.pubkey,
            protocol_owner_state: protocol_owner_state.pubkey,
            new_admin: new_admin.pubkey,
            protocol_staking_admin_state: protocol_staking_admin_state.pubkey,
        })
    }
}
