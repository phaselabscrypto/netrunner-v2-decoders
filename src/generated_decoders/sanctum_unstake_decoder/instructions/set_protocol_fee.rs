use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xadef53f2882b90d9")]
pub struct SetProtocolFee {
    pub protocol_fee: ProtocolFee,
}

pub struct SetProtocolFeeInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub protocol_fee_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetProtocolFee {
    type ArrangedAccounts = SetProtocolFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let protocol_fee_account = accounts.get(1)?;

        Some(SetProtocolFeeInstructionAccounts {
            authority: authority.pubkey,
            protocol_fee_account: protocol_fee_account.pubkey,
        })
    }
}
