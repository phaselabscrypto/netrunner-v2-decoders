
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x2af2426ae40a6f9c")]
pub struct TransferAdmin{
    pub params: TransferAdminParams,
}

pub struct TransferAdminInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub new_admin: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferAdmin {
    type ArrangedAccounts = TransferAdminInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let admin = accounts.get(0)?;
        let new_admin = accounts.get(1)?;
        let perpetuals = accounts.get(2)?;

        Some(TransferAdminInstructionAccounts {
            admin: admin.pubkey,
            new_admin: new_admin.pubkey,
            perpetuals: perpetuals.pubkey,
        })
    }
}