
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa6895ccc91e018da")]
pub struct OperatorSetCustodyConfig{
    pub params: OperatorSetCustodyConfigParams,
}

pub struct OperatorSetCustodyConfigInstructionAccounts {
    pub operator: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OperatorSetCustodyConfig {
    type ArrangedAccounts = OperatorSetCustodyConfigInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let operator = accounts.get(0)?;
        let custody = accounts.get(1)?;

        Some(OperatorSetCustodyConfigInstructionAccounts {
            operator: operator.pubkey,
            custody: custody.pubkey,
        })
    }
}