use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00c9e2e47f2d456e")]
pub struct ClaimStandardCreatorTradingFeeProtocolFees {
    pub amount: u64,
}

pub struct ClaimStandardCreatorTradingFeeProtocolFeesInstructionAccounts {
    pub swap: solana_sdk::pubkey::Pubkey,
    pub protocol_admin: solana_sdk::pubkey::Pubkey,
    pub protocol_admin_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimStandardCreatorTradingFeeProtocolFees {
    type ArrangedAccounts = ClaimStandardCreatorTradingFeeProtocolFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let swap = accounts.get(0)?;
        let protocol_admin = accounts.get(1)?;
        let protocol_admin_state = accounts.get(2)?;

        Some(
            ClaimStandardCreatorTradingFeeProtocolFeesInstructionAccounts {
                swap: swap.pubkey,
                protocol_admin: protocol_admin.pubkey,
                protocol_admin_state: protocol_admin_state.pubkey,
            },
        )
    }
}
