use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x2d52b64ba551793c")]
pub struct ScoreVars {
    pub update_authority_master: solana_sdk::pubkey::Pubkey,
    pub fuel_mint: solana_sdk::pubkey::Pubkey,
    pub food_mint: solana_sdk::pubkey::Pubkey,
    pub arms_mint: solana_sdk::pubkey::Pubkey,
    pub toolkit_mint: solana_sdk::pubkey::Pubkey,
}
