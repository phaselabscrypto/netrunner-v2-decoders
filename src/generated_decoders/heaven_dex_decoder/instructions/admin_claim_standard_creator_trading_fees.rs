use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb627a819603f4c11")]
pub struct AdminClaimStandardCreatorTradingFees {
    pub amount: u64,
}

pub struct AdminClaimStandardCreatorTradingFeesInstructionAccounts {
    pub swap: solana_sdk::pubkey::Pubkey,
    pub protocol_admin: solana_sdk::pubkey::Pubkey,
    pub protocol_admin_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminClaimStandardCreatorTradingFees {
    type ArrangedAccounts = AdminClaimStandardCreatorTradingFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let swap = accounts.get(0)?;
        let protocol_admin = accounts.get(1)?;
        let protocol_admin_state = accounts.get(2)?;

        Some(AdminClaimStandardCreatorTradingFeesInstructionAccounts {
            swap: swap.pubkey,
            protocol_admin: protocol_admin.pubkey,
            protocol_admin_state: protocol_admin_state.pubkey,
        })
    }
}
