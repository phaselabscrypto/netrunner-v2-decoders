use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xee4c24da84b1e0e9")]
pub struct CancelBuy {
    pub buyer_price: u64,
    pub token_size: u64,
    pub buyer_state_expiry: i64,
}

pub struct CancelBuyInstructionAccounts {
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub notary: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub buyer_trade_state: solana_sdk::pubkey::Pubkey,
    pub buyer_referral: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelBuy {
    type ArrangedAccounts = CancelBuyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let wallet = accounts.get(0)?;
        let notary = accounts.get(1)?;
        let token_mint = accounts.get(2)?;
        let authority = accounts.get(3)?;
        let auction_house = accounts.get(4)?;
        let buyer_trade_state = accounts.get(5)?;
        let buyer_referral = accounts.get(6)?;

        Some(CancelBuyInstructionAccounts {
            wallet: wallet.pubkey,
            notary: notary.pubkey,
            token_mint: token_mint.pubkey,
            authority: authority.pubkey,
            auction_house: auction_house.pubkey,
            buyer_trade_state: buyer_trade_state.pubkey,
            buyer_referral: buyer_referral.pubkey,
        })
    }
}
