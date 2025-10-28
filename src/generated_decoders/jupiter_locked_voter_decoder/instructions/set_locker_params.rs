use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6a278454fe4da1a9")]
pub struct SetLockerParams {
    pub params: LockerParams,
}

pub struct SetLockerParamsInstructionAccounts {
    pub locker: solana_sdk::pubkey::Pubkey,
    pub governor: solana_sdk::pubkey::Pubkey,
    pub smart_wallet: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetLockerParams {
    type ArrangedAccounts = SetLockerParamsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [locker, governor, smart_wallet, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetLockerParamsInstructionAccounts {
            locker: locker.pubkey,
            governor: governor.pubkey,
            smart_wallet: smart_wallet.pubkey,
        })
    }
}
