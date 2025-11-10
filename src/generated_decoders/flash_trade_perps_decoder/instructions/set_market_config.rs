use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x80edd83b7a3e9c1e")]
pub struct SetMarketConfig {
    pub params: SetMarketConfigParams,
}

pub struct SetMarketConfigInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub multisig: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub target_custody: solana_sdk::pubkey::Pubkey,
    pub collateral_custody: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetMarketConfig {
    type ArrangedAccounts = SetMarketConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, multisig, market, target_custody, collateral_custody, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SetMarketConfigInstructionAccounts {
            admin: admin.pubkey,
            multisig: multisig.pubkey,
            market: market.pubkey,
            target_custody: target_custody.pubkey,
            collateral_custody: collateral_custody.pubkey,
        })
    }
}
