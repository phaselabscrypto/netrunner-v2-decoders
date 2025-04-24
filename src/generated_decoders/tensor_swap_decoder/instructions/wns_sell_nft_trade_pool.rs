use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa9af7d5801108207")]
pub struct WnsSellNftTradePool {
    pub config: PoolConfig,
    pub min_price: u64,
}

pub struct WnsSellNftTradePoolInstructionAccounts {
    pub shared: solana_sdk::pubkey::Pubkey,
    pub nft_escrow: solana_sdk::pubkey::Pubkey,
    pub nft_receipt: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
    pub approve_account: solana_sdk::pubkey::Pubkey,
    pub distribution: solana_sdk::pubkey::Pubkey,
    pub wns_program: solana_sdk::pubkey::Pubkey,
    pub distribution_program: solana_sdk::pubkey::Pubkey,
    pub extra_metas: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WnsSellNftTradePool {
    type ArrangedAccounts = WnsSellNftTradePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let shared = accounts.get(0)?;
        let nft_escrow = accounts.get(1)?;
        let nft_receipt = accounts.get(2)?;
        let token_program = accounts.get(3)?;
        let associated_token_program = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let margin_account = accounts.get(6)?;
        let taker_broker = accounts.get(7)?;
        let approve_account = accounts.get(8)?;
        let distribution = accounts.get(9)?;
        let wns_program = accounts.get(10)?;
        let distribution_program = accounts.get(11)?;
        let extra_metas = accounts.get(12)?;

        Some(WnsSellNftTradePoolInstructionAccounts {
            shared: shared.pubkey,
            nft_escrow: nft_escrow.pubkey,
            nft_receipt: nft_receipt.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            margin_account: margin_account.pubkey,
            taker_broker: taker_broker.pubkey,
            approve_account: approve_account.pubkey,
            distribution: distribution.pubkey,
            wns_program: wns_program.pubkey,
            distribution_program: distribution_program.pubkey,
            extra_metas: extra_metas.pubkey,
        })
    }
}
