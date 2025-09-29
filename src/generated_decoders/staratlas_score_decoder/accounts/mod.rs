 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::ScoreDecoder; 
pub mod score_vars; 
pub mod score_vars_ship; 
pub mod ship_staking; 

pub enum ScoreAccount { 
        ScoreVars(score_vars::ScoreVars), 
        ScoreVarsShip(score_vars_ship::ScoreVarsShip), 
        ShipStaking(ship_staking::ShipStaking), 
}


impl<'a> AccountDecoder<'a> for ScoreDecoder { 
    type AccountType = ScoreAccount;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = score_vars::ScoreVars::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: ScoreAccount::ScoreVars(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = score_vars_ship::ScoreVarsShip::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: ScoreAccount::ScoreVarsShip(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = ship_staking::ShipStaking::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: ScoreAccount::ShipStaking(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}