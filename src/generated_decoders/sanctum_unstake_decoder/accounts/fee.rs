use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x183796faa81b65b2")]
pub struct Fee {
    pub fee: FeeEnum,
}
