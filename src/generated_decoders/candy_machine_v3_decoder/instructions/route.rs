use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe517cb977ae3ad2a")]
pub struct Route {
    pub args: RouteArgs,
    pub label: Option<String>,
}

pub struct RouteInstructionAccounts {
    pub candy_guard: solana_sdk::pubkey::Pubkey,
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Route {
    type ArrangedAccounts = RouteInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let candy_guard = accounts.get(0)?;
        let candy_machine = accounts.get(1)?;
        let payer = accounts.get(2)?;

        Some(RouteInstructionAccounts {
            candy_guard: candy_guard.pubkey,
            candy_machine: candy_machine.pubkey,
            payer: payer.pubkey,
        })
    }
}
