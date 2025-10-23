use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xda0c583a962c9848")]
pub struct DeactivateProtocolLookupTable {
    pub version: u64,
}

pub struct DeactivateProtocolLookupTableInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub address_lookup_program: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub lookup_table: solana_sdk::pubkey::Pubkey,
    pub protocol_owner_state: solana_sdk::pubkey::Pubkey,
    pub current_owner: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeactivateProtocolLookupTable {
    type ArrangedAccounts = DeactivateProtocolLookupTableInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let system_program = accounts.get(1)?;
        let address_lookup_program = accounts.get(2)?;
        let authority = accounts.get(3)?;
        let lookup_table = accounts.get(4)?;
        let protocol_owner_state = accounts.get(5)?;
        let current_owner = accounts.get(6)?;

        Some(DeactivateProtocolLookupTableInstructionAccounts {
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            address_lookup_program: address_lookup_program.pubkey,
            authority: authority.pubkey,
            lookup_table: lookup_table.pubkey,
            protocol_owner_state: protocol_owner_state.pubkey,
            current_owner: current_owner.pubkey,
        })
    }
}
