use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc6c682cba35faf4b")]
pub struct CancelSell {
    pub buyer_price: u64,
    pub token_size: u64,
    pub seller_state_expiry: i64,
}

pub struct CancelSellInstructionAccounts {
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub notary: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub seller_trade_state: solana_sdk::pubkey::Pubkey,
    pub seller_referral: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelSell {
    type ArrangedAccounts = CancelSellInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let wallet = accounts.get(0)?;
        let notary = accounts.get(1)?;
        let token_account = accounts.get(2)?;
        let token_mint = accounts.get(3)?;
        let authority = accounts.get(4)?;
        let auction_house = accounts.get(5)?;
        let seller_trade_state = accounts.get(6)?;
        let seller_referral = accounts.get(7)?;
        let token_program = accounts.get(8)?;

        Some(CancelSellInstructionAccounts {
            wallet: wallet.pubkey,
            notary: notary.pubkey,
            token_account: token_account.pubkey,
            token_mint: token_mint.pubkey,
            authority: authority.pubkey,
            auction_house: auction_house.pubkey,
            seller_trade_state: seller_trade_state.pubkey,
            seller_referral: seller_referral.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
