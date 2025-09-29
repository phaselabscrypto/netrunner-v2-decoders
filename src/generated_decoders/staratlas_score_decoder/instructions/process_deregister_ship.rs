

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0d3eb6dbb400c34d")]
pub struct ProcessDeregisterShip{
    pub scorevars_bump: u8,
    pub scorevars_ship_bump: u8,
}

pub struct ProcessDeregisterShipInstructionAccounts {
    pub update_authority_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_ship_account: solana_sdk::pubkey::Pubkey,
    pub ship_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessDeregisterShip {
    type ArrangedAccounts = ProcessDeregisterShipInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            update_authority_account,
            score_vars_account,
            score_vars_ship_account,
            ship_mint,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(ProcessDeregisterShipInstructionAccounts {
            update_authority_account: update_authority_account.pubkey,
            score_vars_account: score_vars_account.pubkey,
            score_vars_ship_account: score_vars_ship_account.pubkey,
            ship_mint: ship_mint.pubkey,
        })
    }
}