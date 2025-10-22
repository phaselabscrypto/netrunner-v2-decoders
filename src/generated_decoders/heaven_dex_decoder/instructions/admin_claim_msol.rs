use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7c303cc7cb312429")]
pub struct AdminClaimMsol {
    pub version: u16,
    pub ticket_number: u32,
}

pub struct AdminClaimMsolInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub protocol_config_state: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub protocol_staking_admin_state: solana_sdk::pubkey::Pubkey,
    pub msol_ticket: solana_sdk::pubkey::Pubkey,
    pub msol_mint: solana_sdk::pubkey::Pubkey,
    pub msol_ticket_sol_spent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminClaimMsol {
    type ArrangedAccounts = AdminClaimMsolInstructionAccounts;

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
        let msol_ticket = accounts.get(7)?;
        let msol_mint = accounts.get(8)?;
        let msol_ticket_sol_spent = accounts.get(9)?;

        Some(AdminClaimMsolInstructionAccounts {
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            payer: payer.pubkey,
            admin: admin.pubkey,
            protocol_config_state: protocol_config_state.pubkey,
            system_program: system_program.pubkey,
            protocol_staking_admin_state: protocol_staking_admin_state.pubkey,
            msol_ticket: msol_ticket.pubkey,
            msol_mint: msol_mint.pubkey,
            msol_ticket_sol_spent: msol_ticket_sol_spent.pubkey,
        })
    }
}
