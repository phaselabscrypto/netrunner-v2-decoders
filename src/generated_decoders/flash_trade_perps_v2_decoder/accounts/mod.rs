use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::FlashTradePerpsV2Decoder;
pub mod custody;
pub mod custom_oracle;
pub mod flp_stake;
pub mod market;
pub mod multisig;
pub mod order;
pub mod perpetuals;
pub mod pool;
pub mod position;
pub mod protocol_vault;
pub mod rebate_vault;
pub mod referral;
pub mod token_stake;
pub mod token_vault;
pub mod trading;

pub enum FlashTradePerpsV2Account {
    Custody(custody::Custody),
    FlpStake(flp_stake::FlpStake),
    Market(market::Market),
    Multisig(multisig::Multisig),
    CustomOracle(custom_oracle::CustomOracle),
    Order(order::Order),
    Perpetuals(perpetuals::Perpetuals),
    Pool(pool::Pool),
    Position(position::Position),
    ProtocolVault(protocol_vault::ProtocolVault),
    RebateVault(rebate_vault::RebateVault),
    Referral(referral::Referral),
    TokenStake(token_stake::TokenStake),
    TokenVault(token_vault::TokenVault),
    Trading(trading::Trading),
}

impl<'a> AccountDecoder<'a> for FlashTradePerpsV2Decoder {
    type AccountType = FlashTradePerpsV2Account;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = custody::Custody::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FlashTradePerpsV2Account::Custody(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = flp_stake::FlpStake::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FlashTradePerpsV2Account::FlpStake(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = market::Market::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FlashTradePerpsV2Account::Market(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = multisig::Multisig::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FlashTradePerpsV2Account::Multisig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            custom_oracle::CustomOracle::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FlashTradePerpsV2Account::CustomOracle(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = order::Order::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FlashTradePerpsV2Account::Order(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = perpetuals::Perpetuals::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FlashTradePerpsV2Account::Perpetuals(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = pool::Pool::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FlashTradePerpsV2Account::Pool(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = position::Position::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FlashTradePerpsV2Account::Position(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            protocol_vault::ProtocolVault::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FlashTradePerpsV2Account::ProtocolVault(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            rebate_vault::RebateVault::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FlashTradePerpsV2Account::RebateVault(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = referral::Referral::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FlashTradePerpsV2Account::Referral(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = token_stake::TokenStake::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FlashTradePerpsV2Account::TokenStake(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = token_vault::TokenVault::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FlashTradePerpsV2Account::TokenVault(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = trading::Trading::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FlashTradePerpsV2Account::Trading(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
