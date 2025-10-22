use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8cca39c361d5a813")]
pub struct AdminMintMsol {
    pub version: u16,
    pub amount: u64,
}

pub struct AdminMintMsolInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub protocol_config_state: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub protocol_staking_admin_state: solana_sdk::pubkey::Pubkey,
    pub address_lookup_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account_info: solana_sdk::pubkey::Pubkey,
    pub temp_sol_holder: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminMintMsol {
    type ArrangedAccounts = AdminMintMsolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let associated_token_program = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let admin = accounts.get(3)?;
        let protocol_config_state = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let protocol_staking_admin_state = accounts.get(6)?;
        let address_lookup_program = accounts.get(7)?;
        let instruction_sysvar_account_info = accounts.get(8)?;
        let temp_sol_holder = accounts.get(9)?;

        Some(AdminMintMsolInstructionAccounts {
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            payer: payer.pubkey,
            admin: admin.pubkey,
            protocol_config_state: protocol_config_state.pubkey,
            system_program: system_program.pubkey,
            protocol_staking_admin_state: protocol_staking_admin_state.pubkey,
            address_lookup_program: address_lookup_program.pubkey,
            instruction_sysvar_account_info: instruction_sysvar_account_info.pubkey,
            temp_sol_holder: temp_sol_holder.pubkey,
        })
    }
}
