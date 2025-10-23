use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x18a3dcabe1dea6f8")]
pub struct AdminClaimStakingRewards {
    pub version: u16,
    pub amount: u64,
}

pub struct AdminClaimStakingRewardsInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub protocol_config_state: solana_sdk::pubkey::Pubkey,
    pub protocol_config_wsol_vault: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub protocol_staking_admin_state: solana_sdk::pubkey::Pubkey,
    pub wsol_token_vault: solana_sdk::pubkey::Pubkey,
    pub wsol_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminClaimStakingRewards {
    type ArrangedAccounts = AdminClaimStakingRewardsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let associated_token_program = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let admin = accounts.get(3)?;
        let protocol_config_state = accounts.get(4)?;
        let protocol_config_wsol_vault = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let protocol_staking_admin_state = accounts.get(7)?;
        let wsol_token_vault = accounts.get(8)?;
        let wsol_mint = accounts.get(9)?;

        Some(AdminClaimStakingRewardsInstructionAccounts {
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            payer: payer.pubkey,
            admin: admin.pubkey,
            protocol_config_state: protocol_config_state.pubkey,
            protocol_config_wsol_vault: protocol_config_wsol_vault.pubkey,
            system_program: system_program.pubkey,
            protocol_staking_admin_state: protocol_staking_admin_state.pubkey,
            wsol_token_vault: wsol_token_vault.pubkey,
            wsol_mint: wsol_mint.pubkey,
        })
    }
}
