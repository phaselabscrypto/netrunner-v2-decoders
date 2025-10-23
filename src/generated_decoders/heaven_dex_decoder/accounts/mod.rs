use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::HeavenDexDecoder;
pub mod liquidity_pool_state;
pub mod msol_ticket_sol_spent;
pub mod protocol_admin_state;
pub mod protocol_config;
pub mod protocol_owner_state;

pub enum HeavenDexAccount {
    LiquidityPoolState(liquidity_pool_state::LiquidityPoolState),
    MsolTicketSolSpent(msol_ticket_sol_spent::MsolTicketSolSpent),
    ProtocolAdminState(protocol_admin_state::ProtocolAdminState),
    ProtocolConfig(protocol_config::ProtocolConfig),
    ProtocolOwnerState(protocol_owner_state::ProtocolOwnerState),
}

impl<'a> AccountDecoder<'a> for HeavenDexDecoder {
    type AccountType = HeavenDexAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            liquidity_pool_state::LiquidityPoolState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: HeavenDexAccount::LiquidityPoolState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            msol_ticket_sol_spent::MsolTicketSolSpent::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: HeavenDexAccount::MsolTicketSolSpent(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            protocol_admin_state::ProtocolAdminState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: HeavenDexAccount::ProtocolAdminState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            protocol_config::ProtocolConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: HeavenDexAccount::ProtocolConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            protocol_owner_state::ProtocolOwnerState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: HeavenDexAccount::ProtocolOwnerState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
