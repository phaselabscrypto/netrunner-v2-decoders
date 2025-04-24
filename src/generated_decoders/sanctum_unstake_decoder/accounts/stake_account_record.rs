use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x90cdb7f103fad0d7")]
pub struct StakeAccountRecord {
    pub lamports_at_creation: u64,
}
