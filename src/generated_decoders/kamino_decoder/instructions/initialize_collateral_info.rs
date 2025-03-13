use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4a3dd84cf45b1277")]
pub struct InitializeCollateralInfo {}

pub struct InitializeCollateralInfoInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub coll_info: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeCollateralInfo {
    type ArrangedAccounts = InitializeCollateralInfoInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, global_config, coll_info, system_program, _remaining @ ..] =
            accounts.as_slice()
        else {
            return None;
        };

        Some(InitializeCollateralInfoInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            global_config: global_config.pubkey,
            coll_info: coll_info.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
