use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7a822112d32ca13a")]
pub struct ClosePositionRequest3 {}

pub struct ClosePositionRequest3InstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub owner_ata: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position_request: solana_sdk::pubkey::Pubkey,
    pub position_request_ata: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePositionRequest3 {
    type ArrangedAccounts = ClosePositionRequest3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, owner, owner_ata, pool, position_request, position_request_ata, position, custody, mint, token_program, system_program, associated_token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ClosePositionRequest3InstructionAccounts {
            keeper: keeper.pubkey,
            owner: owner.pubkey,
            owner_ata: owner_ata.pubkey,
            pool: pool.pubkey,
            position_request: position_request.pubkey,
            position_request_ata: position_request_ata.pubkey,
            position: position.pubkey,
            custody: custody.pubkey,
            mint: mint.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
