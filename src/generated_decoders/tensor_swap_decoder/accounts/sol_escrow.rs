use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x4bc7fa3ff4d1eb78")]
pub struct SolEscrow {}
