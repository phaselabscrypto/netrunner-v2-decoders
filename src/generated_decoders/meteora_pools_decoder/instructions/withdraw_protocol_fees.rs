

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0b44a56212d08649")]
pub struct WithdrawProtocolFees{
}

pub struct WithdrawProtocolFeesInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp: solana_sdk::pubkey::Pubkey,
    pub protocol_token_a_fee: solana_sdk::pubkey::Pubkey,
    pub protocol_token_b_fee: solana_sdk::pubkey::Pubkey,
    pub treasury_token_a: solana_sdk::pubkey::Pubkey,
    pub treasury_token_b: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawProtocolFees {
    type ArrangedAccounts = WithdrawProtocolFeesInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;
        let a_vault_lp = accounts.get(1)?;
        let protocol_token_a_fee = accounts.get(2)?;
        let protocol_token_b_fee = accounts.get(3)?;
        let treasury_token_a = accounts.get(4)?;
        let treasury_token_b = accounts.get(5)?;
        let token_program = accounts.get(6)?;

        Some(WithdrawProtocolFeesInstructionAccounts {
            pool: pool.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            protocol_token_a_fee: protocol_token_a_fee.pubkey,
            protocol_token_b_fee: protocol_token_b_fee.pubkey,
            treasury_token_a: treasury_token_a.pubkey,
            treasury_token_b: treasury_token_b.pubkey,
            token_program: token_program.pubkey,
        })
    }
}