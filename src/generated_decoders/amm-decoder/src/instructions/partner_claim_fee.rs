

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x3935b01e7b463440")]
pub struct PartnerClaimFee{
    pub max_amount_a: u64,
    pub max_amount_b: u64,
}

pub struct PartnerClaimFeeInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp: solana_sdk::pubkey::Pubkey,
    pub protocol_token_a_fee: solana_sdk::pubkey::Pubkey,
    pub protocol_token_b_fee: solana_sdk::pubkey::Pubkey,
    pub partner_token_a: solana_sdk::pubkey::Pubkey,
    pub partner_token_b: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub partner_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PartnerClaimFee {
    type ArrangedAccounts = PartnerClaimFeeInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;
        let a_vault_lp = accounts.get(1)?;
        let protocol_token_a_fee = accounts.get(2)?;
        let protocol_token_b_fee = accounts.get(3)?;
        let partner_token_a = accounts.get(4)?;
        let partner_token_b = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let partner_authority = accounts.get(7)?;

        Some(PartnerClaimFeeInstructionAccounts {
            pool: pool.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            protocol_token_a_fee: protocol_token_a_fee.pubkey,
            protocol_token_b_fee: protocol_token_b_fee.pubkey,
            partner_token_a: partner_token_a.pubkey,
            partner_token_b: partner_token_b.pubkey,
            token_program: token_program.pubkey,
            partner_authority: partner_authority.pubkey,
        })
    }
}