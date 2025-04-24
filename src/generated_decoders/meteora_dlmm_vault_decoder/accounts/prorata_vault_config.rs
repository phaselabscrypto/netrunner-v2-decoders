use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x5dd6cd6877093398")]
pub struct ProrataVaultConfig {
    pub max_buying_cap: u64,
    pub start_vesting_duration: u64,
    pub end_vesting_duration: u64,
    pub escrow_fee: u64,
    pub activation_type: u8,
    pub padding: [u8; 191],
}
