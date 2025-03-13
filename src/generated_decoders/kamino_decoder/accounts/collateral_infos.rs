use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x7fd234e24aa96f09")]
pub struct CollateralInfos {
    pub infos: [CollateralInfo; 256],
}
