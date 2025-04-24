use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xb4338af7f0ad774f")]
pub struct ScopeChainAccount {
    pub chain_array: [[u16; 4]; 512],
}
