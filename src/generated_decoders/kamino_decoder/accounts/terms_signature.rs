use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xc5ad885bb6317113")]
pub struct TermsSignature {
    pub signature: [u8; 64],
}
