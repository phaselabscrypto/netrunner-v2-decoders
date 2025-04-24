use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x83e2a186e984f39f")]
pub struct WnsDelist {}

pub struct WnsDelistInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub nft_dest: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub nft_escrow: solana_sdk::pubkey::Pubkey,
    pub single_listing: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub approve_account: solana_sdk::pubkey::Pubkey,
    pub distribution: solana_sdk::pubkey::Pubkey,
    pub wns_program: solana_sdk::pubkey::Pubkey,
    pub distribution_program: solana_sdk::pubkey::Pubkey,
    pub extra_metas: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WnsDelist {
    type ArrangedAccounts = WnsDelistInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let nft_dest = accounts.get(1)?;
        let nft_mint = accounts.get(2)?;
        let nft_escrow = accounts.get(3)?;
        let single_listing = accounts.get(4)?;
        let owner = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let rent = accounts.get(8)?;
        let associated_token_program = accounts.get(9)?;
        let payer = accounts.get(10)?;
        let approve_account = accounts.get(11)?;
        let distribution = accounts.get(12)?;
        let wns_program = accounts.get(13)?;
        let distribution_program = accounts.get(14)?;
        let extra_metas = accounts.get(15)?;

        Some(WnsDelistInstructionAccounts {
            tswap: tswap.pubkey,
            nft_dest: nft_dest.pubkey,
            nft_mint: nft_mint.pubkey,
            nft_escrow: nft_escrow.pubkey,
            single_listing: single_listing.pubkey,
            owner: owner.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            associated_token_program: associated_token_program.pubkey,
            payer: payer.pubkey,
            approve_account: approve_account.pubkey,
            distribution: distribution.pubkey,
            wns_program: wns_program.pubkey,
            distribution_program: distribution_program.pubkey,
            extra_metas: extra_metas.pubkey,
        })
    }
}
