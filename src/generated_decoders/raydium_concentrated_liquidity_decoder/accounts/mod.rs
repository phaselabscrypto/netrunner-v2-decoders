 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::AmmV3Decoder; 
pub mod amm_config; 
pub mod operation_state; 
pub mod observation_state; 
pub mod personal_position_state; 
pub mod pool_state; 
pub mod protocol_position_state; 
pub mod tick_array_state; 
pub mod tick_array_bitmap_extension; 

pub enum AmmV3Account { 
        AmmConfig(amm_config::AmmConfig), 
        OperationState(operation_state::OperationState), 
        ObservationState(observation_state::ObservationState), 
        PersonalPositionState(personal_position_state::PersonalPositionState), 
        PoolState(pool_state::PoolState), 
        ProtocolPositionState(protocol_position_state::ProtocolPositionState), 
        TickArrayState(tick_array_state::TickArrayState), 
        TickArrayBitmapExtension(tick_array_bitmap_extension::TickArrayBitmapExtension), 
}


impl<'a> AccountDecoder<'a> for AmmV3Decoder { 
    type AccountType = AmmV3Account;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = amm_config::AmmConfig::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: AmmV3Account::AmmConfig(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = operation_state::OperationState::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: AmmV3Account::OperationState(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = observation_state::ObservationState::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: AmmV3Account::ObservationState(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = personal_position_state::PersonalPositionState::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: AmmV3Account::PersonalPositionState(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = pool_state::PoolState::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: AmmV3Account::PoolState(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = protocol_position_state::ProtocolPositionState::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: AmmV3Account::ProtocolPositionState(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = tick_array_state::TickArrayState::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: AmmV3Account::TickArrayState(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = tick_array_bitmap_extension::TickArrayBitmapExtension::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: AmmV3Account::TickArrayBitmapExtension(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}