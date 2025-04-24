use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::M2Decoder;
pub mod auction_house;
pub mod buyer_trade_state;
pub mod buyer_trade_state_v2;
pub mod seller_trade_state;
pub mod seller_trade_state_v2;

pub enum M2Account {
    BuyerTradeState(buyer_trade_state::BuyerTradeState),
    SellerTradeState(seller_trade_state::SellerTradeState),
    SellerTradeStateV2(seller_trade_state_v2::SellerTradeStateV2),
    AuctionHouse(auction_house::AuctionHouse),
    BuyerTradeStateV2(buyer_trade_state_v2::BuyerTradeStateV2),
}

impl<'a> AccountDecoder<'a> for M2Decoder {
    type AccountType = M2Account;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            buyer_trade_state::BuyerTradeState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: M2Account::BuyerTradeState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            seller_trade_state::SellerTradeState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: M2Account::SellerTradeState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            seller_trade_state_v2::SellerTradeStateV2::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: M2Account::SellerTradeStateV2(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            auction_house::AuctionHouse::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: M2Account::AuctionHouse(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            buyer_trade_state_v2::BuyerTradeStateV2::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: M2Account::BuyerTradeStateV2(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
