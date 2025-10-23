use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x754fa4cb7e4816f6")]
pub struct AdminWithdrawTransferFee {
    pub protocol_config_version: u16,
}

pub struct AdminWithdrawTransferFeeInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub receiver: solana_sdk::pubkey::Pubkey,
    pub protocol_fee_admin_state: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub protocol_config: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminWithdrawTransferFee {
    type ArrangedAccounts = AdminWithdrawTransferFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let receiver = accounts.get(2)?;
        let protocol_fee_admin_state = accounts.get(3)?;
        let admin = accounts.get(4)?;
        let protocol_config = accounts.get(5)?;

        Some(AdminWithdrawTransferFeeInstructionAccounts {
            token_program: token_program.pubkey,
            mint: mint.pubkey,
            receiver: receiver.pubkey,
            protocol_fee_admin_state: protocol_fee_admin_state.pubkey,
            admin: admin.pubkey,
            protocol_config: protocol_config.pubkey,
        })
    }
}
