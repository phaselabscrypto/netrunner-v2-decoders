use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3db7b9bc52518dc5")]
pub struct UpdateAmmAdminGroup {
    pub params: UpdateAdminGroupParams,
}

pub struct UpdateAmmAdminGroupInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub admin_group: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateAmmAdminGroup {
    type ArrangedAccounts = UpdateAmmAdminGroupInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let admin_group = accounts.get(1)?;

        Some(UpdateAmmAdminGroupInstructionAccounts {
            payer: payer.pubkey,
            admin_group: admin_group.pubkey,
        })
    }
}
