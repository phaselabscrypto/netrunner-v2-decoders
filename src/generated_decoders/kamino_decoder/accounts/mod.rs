use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::YvaultsDecoder;
pub mod collateral_infos;
pub mod global_config;
pub mod personal_position_state;
pub mod pool_state;
pub mod position;
pub mod protocol_position_state;
pub mod scope_chain_account;
pub mod terms_signature;
pub mod whirlpool;
pub mod whirlpool_strategy;

pub enum YvaultsAccount {
    Whirlpool(whirlpool::Whirlpool),
    Position(position::Position),
    PoolState(pool_state::PoolState),
    PersonalPositionState(personal_position_state::PersonalPositionState),
    ProtocolPositionState(protocol_position_state::ProtocolPositionState),
    WhirlpoolStrategy(whirlpool_strategy::WhirlpoolStrategy),
    GlobalConfig(global_config::GlobalConfig),
    CollateralInfos(collateral_infos::CollateralInfos),
    ScopeChainAccount(scope_chain_account::ScopeChainAccount),
    TermsSignature(terms_signature::TermsSignature),
}

impl<'a> AccountDecoder<'a> for YvaultsDecoder {
    type AccountType = YvaultsAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = whirlpool::Whirlpool::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: YvaultsAccount::Whirlpool(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = position::Position::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: YvaultsAccount::Position(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = pool_state::PoolState::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: YvaultsAccount::PoolState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            personal_position_state::PersonalPositionState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: YvaultsAccount::PersonalPositionState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            protocol_position_state::ProtocolPositionState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: YvaultsAccount::ProtocolPositionState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            whirlpool_strategy::WhirlpoolStrategy::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: YvaultsAccount::WhirlpoolStrategy(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            global_config::GlobalConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: YvaultsAccount::GlobalConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            collateral_infos::CollateralInfos::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: YvaultsAccount::CollateralInfos(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            scope_chain_account::ScopeChainAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: YvaultsAccount::ScopeChainAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            terms_signature::TermsSignature::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: YvaultsAccount::TermsSignature(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
