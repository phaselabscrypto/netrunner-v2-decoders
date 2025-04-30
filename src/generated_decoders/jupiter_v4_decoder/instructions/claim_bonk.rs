use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdfafbad401ce52db")]
pub struct ClaimBonk {}

pub struct ClaimBonkInstructionAccounts {
    pub open_orders: solana_sdk::pubkey::Pubkey,
    pub bonk_mint: solana_sdk::pubkey::Pubkey,
    pub open_orders_bonk_token_account: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub open_orders_owner: solana_sdk::pubkey::Pubkey,
    pub claimer_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimBonk {
    type ArrangedAccounts = ClaimBonkInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let open_orders = accounts.get(0)?;
        let bonk_mint = accounts.get(1)?;
        let open_orders_bonk_token_account = accounts.get(2)?;
        let market = accounts.get(3)?;
        let open_orders_owner = accounts.get(4)?;
        let claimer_token_account = accounts.get(5)?;
        let token_program = accounts.get(6)?;

        Some(ClaimBonkInstructionAccounts {
            open_orders: open_orders.pubkey,
            bonk_mint: bonk_mint.pubkey,
            open_orders_bonk_token_account: open_orders_bonk_token_account.pubkey,
            market: market.pubkey,
            open_orders_owner: open_orders_owner.pubkey,
            claimer_token_account: claimer_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
