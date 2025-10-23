use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x42c43e867c95fa42")]
pub struct MsolTicketSolSpent {
    pub cost_basis: u64,
    pub msol_unstaked: u64,
}
