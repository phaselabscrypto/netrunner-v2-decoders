
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x326a426863769158")]
pub struct ChangeAuthority{
    pub data: ChangeAuthorityData,
}

pub struct ChangeAuthorityInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub admin_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ChangeAuthority {
    type ArrangedAccounts = ChangeAuthorityInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let admin_authority = accounts.get(1)?;

        Some(ChangeAuthorityInstructionAccounts {
            state: state.pubkey,
            admin_authority: admin_authority.pubkey,
        })
    }
}