use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4749389bca8e6496")]
pub struct SetLpTokenMetadata {
    pub data: DataV2LpToken,
}

pub struct SetLpTokenMetadataInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub pool_account: solana_sdk::pubkey::Pubkey,
    pub pool_sol_reserves: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetLpTokenMetadata {
    type ArrangedAccounts = SetLpTokenMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let fee_authority = accounts.get(1)?;
        let pool_account = accounts.get(2)?;
        let pool_sol_reserves = accounts.get(3)?;
        let lp_mint = accounts.get(4)?;
        let metadata = accounts.get(5)?;
        let metadata_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let rent = accounts.get(8)?;

        Some(SetLpTokenMetadataInstructionAccounts {
            payer: payer.pubkey,
            fee_authority: fee_authority.pubkey,
            pool_account: pool_account.pubkey,
            pool_sol_reserves: pool_sol_reserves.pubkey,
            lp_mint: lp_mint.pubkey,
            metadata: metadata.pubkey,
            metadata_program: metadata_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
