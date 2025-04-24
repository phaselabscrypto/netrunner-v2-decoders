use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x63f3fc7aa0af8234")]
pub struct FcfsVaultConfig {
    pub max_depositing_cap: u64,
    pub start_vesting_duration: u64,
    pub end_vesting_duration: u64,
    pub depositing_duration_until_last_join_point: u64,
    pub individual_depositing_cap: u64,
    pub escrow_fee: u64,
    pub activation_type: u8,
    pub padding: [u8; 175],
}
