use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8197b5d42fe83a62")]
pub struct MoveProtocolFees {}

pub struct MoveProtocolFeesInstructionAccounts {
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub custody_token_account: solana_sdk::pubkey::Pubkey,
    pub revenue_token_account: solana_sdk::pubkey::Pubkey,
    pub protocol_vault: solana_sdk::pubkey::Pubkey,
    pub protocol_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MoveProtocolFees {
    type ArrangedAccounts = MoveProtocolFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [transfer_authority, perpetuals, token_vault, pool, custody, custody_token_account, revenue_token_account, protocol_vault, protocol_token_account, token_program, event_authority, program, token_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(MoveProtocolFeesInstructionAccounts {
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            token_vault: token_vault.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            custody_token_account: custody_token_account.pubkey,
            revenue_token_account: revenue_token_account.pubkey,
            protocol_vault: protocol_vault.pubkey,
            protocol_token_account: protocol_token_account.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            token_mint: token_mint.pubkey,
        })
    }
}
