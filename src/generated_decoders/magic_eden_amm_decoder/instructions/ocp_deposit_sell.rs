
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x218955854de0aefd")]
pub struct OcpDepositSell{
    pub args: DepositSellArgs,
}

pub struct OcpDepositSellInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub asset_metadata: solana_sdk::pubkey::Pubkey,
    pub asset_mint: solana_sdk::pubkey::Pubkey,
    pub asset_token_account: solana_sdk::pubkey::Pubkey,
    pub sellside_escrow_token_account: solana_sdk::pubkey::Pubkey,
    pub sell_state: solana_sdk::pubkey::Pubkey,
    pub allowlist_aux_account: solana_sdk::pubkey::Pubkey,
    pub ocp_mint_state: solana_sdk::pubkey::Pubkey,
    pub ocp_policy: solana_sdk::pubkey::Pubkey,
    pub ocp_freeze_authority: solana_sdk::pubkey::Pubkey,
    pub ocp_program: solana_sdk::pubkey::Pubkey,
    pub cmt_program: solana_sdk::pubkey::Pubkey,
    pub instructions: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OcpDepositSell {
    type ArrangedAccounts = OcpDepositSellInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let cosigner = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let asset_metadata = accounts.get(3)?;
        let asset_mint = accounts.get(4)?;
        let asset_token_account = accounts.get(5)?;
        let sellside_escrow_token_account = accounts.get(6)?;
        let sell_state = accounts.get(7)?;
        let allowlist_aux_account = accounts.get(8)?;
        let ocp_mint_state = accounts.get(9)?;
        let ocp_policy = accounts.get(10)?;
        let ocp_freeze_authority = accounts.get(11)?;
        let ocp_program = accounts.get(12)?;
        let cmt_program = accounts.get(13)?;
        let instructions = accounts.get(14)?;
        let system_program = accounts.get(15)?;
        let token_program = accounts.get(16)?;
        let associated_token_program = accounts.get(17)?;
        let rent = accounts.get(18)?;

        Some(OcpDepositSellInstructionAccounts {
            owner: owner.pubkey,
            cosigner: cosigner.pubkey,
            pool: pool.pubkey,
            asset_metadata: asset_metadata.pubkey,
            asset_mint: asset_mint.pubkey,
            asset_token_account: asset_token_account.pubkey,
            sellside_escrow_token_account: sellside_escrow_token_account.pubkey,
            sell_state: sell_state.pubkey,
            allowlist_aux_account: allowlist_aux_account.pubkey,
            ocp_mint_state: ocp_mint_state.pubkey,
            ocp_policy: ocp_policy.pubkey,
            ocp_freeze_authority: ocp_freeze_authority.pubkey,
            ocp_program: ocp_program.pubkey,
            cmt_program: cmt_program.pubkey,
            instructions: instructions.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}