use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xe07479ba44a14fec")]
pub struct Multisig {
    pub num_signers: u8,
    pub num_signed: u8,
    pub min_signatures: u8,
    pub instruction_accounts_len: u8,
    pub instruction_data_len: u16,
    pub instruction_hash: u64,
    pub signers: [solana_sdk::pubkey::Pubkey; 6],
    pub signed: [u8; 6],
    pub bump: u8,
}
