 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::AlphaVaultDecoder; 
pub mod escrow; 
pub mod merkle_root_config; 
pub mod prorata_vault_config; 
pub mod fcfs_vault_config; 
pub mod vault; 

pub enum AlphaVaultAccount { 
        Escrow(escrow::Escrow), 
        MerkleRootConfig(merkle_root_config::MerkleRootConfig), 
        ProrataVaultConfig(prorata_vault_config::ProrataVaultConfig), 
        FcfsVaultConfig(fcfs_vault_config::FcfsVaultConfig), 
        Vault(vault::Vault), 
}


impl<'a> AccountDecoder<'a> for AlphaVaultDecoder { 
    type AccountType = AlphaVaultAccount;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = escrow::Escrow::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: AlphaVaultAccount::Escrow(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = merkle_root_config::MerkleRootConfig::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: AlphaVaultAccount::MerkleRootConfig(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = prorata_vault_config::ProrataVaultConfig::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: AlphaVaultAccount::ProrataVaultConfig(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = fcfs_vault_config::FcfsVaultConfig::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: AlphaVaultAccount::FcfsVaultConfig(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = vault::Vault::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: AlphaVaultAccount::Vault(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}