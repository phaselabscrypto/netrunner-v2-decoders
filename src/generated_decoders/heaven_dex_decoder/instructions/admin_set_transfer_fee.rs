use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe8c1a803505cd7b0")]
pub struct AdminSetTransferFee {
    pub protocol_config_version: u16,
    pub transfer_fee_bps: u16,
    pub maximum_fee: u64,
}

pub struct AdminSetTransferFeeInstructionAccounts {
    pub token_a_program: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub current_owner: solana_sdk::pubkey::Pubkey,
    pub protocol_owner_state: solana_sdk::pubkey::Pubkey,
    pub protocol_config: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminSetTransferFee {
    type ArrangedAccounts = AdminSetTransferFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let token_a_program = accounts.get(0)?;
        let token_a_mint = accounts.get(1)?;
        let current_owner = accounts.get(2)?;
        let protocol_owner_state = accounts.get(3)?;
        let protocol_config = accounts.get(4)?;

        Some(AdminSetTransferFeeInstructionAccounts {
            token_a_program: token_a_program.pubkey,
            token_a_mint: token_a_mint.pubkey,
            current_owner: current_owner.pubkey,
            protocol_owner_state: protocol_owner_state.pubkey,
            protocol_config: protocol_config.pubkey,
        })
    }
}
