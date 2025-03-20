

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4c5e832c893da16e")]
pub struct UpdateCollateralInfo{
    pub index: u64,
    pub mode: u64,
    pub value: [u8; 32],
}

pub struct UpdateCollateralInfoInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub token_infos: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateCollateralInfo {
    type ArrangedAccounts = UpdateCollateralInfoInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let global_config = accounts.get(1)?;
        let token_infos = accounts.get(2)?;

        Some(UpdateCollateralInfoInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            global_config: global_config.pubkey,
            token_infos: token_infos.pubkey,
        })
    }
}