use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4c5e832c893da16e")]
pub struct UpdateCollateralInfo {
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

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, global_config, token_infos, _remaining @ ..] = accounts.as_slice() else {
            return None;
        };

        Some(UpdateCollateralInfoInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            global_config: global_config.pubkey,
            token_infos: token_infos.pubkey,
        })
    }
}
