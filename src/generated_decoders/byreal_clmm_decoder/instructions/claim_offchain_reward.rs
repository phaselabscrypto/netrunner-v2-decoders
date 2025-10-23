use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc357dd958dc39213")]
pub struct ClaimOffchainReward {
    pub amount: u64,
}

pub struct ClaimOffchainRewardInstructionAccounts {
    pub claimer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub admin_group: solana_sdk::pubkey::Pubkey,
    pub pool_id: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub claimer_token_account: solana_sdk::pubkey::Pubkey,
    pub reward_vault_token_account: solana_sdk::pubkey::Pubkey,
    pub reward_config: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimOffchainReward {
    type ArrangedAccounts = ClaimOffchainRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let claimer = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let admin_group = accounts.get(2)?;
        let pool_id = accounts.get(3)?;
        let token_mint = accounts.get(4)?;
        let claimer_token_account = accounts.get(5)?;
        let reward_vault_token_account = accounts.get(6)?;
        let reward_config = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let associated_token_program = accounts.get(9)?;

        Some(ClaimOffchainRewardInstructionAccounts {
            claimer: claimer.pubkey,
            authority: authority.pubkey,
            admin_group: admin_group.pubkey,
            pool_id: pool_id.pubkey,
            token_mint: token_mint.pubkey,
            claimer_token_account: claimer_token_account.pubkey,
            reward_vault_token_account: reward_vault_token_account.pubkey,
            reward_config: reward_config.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}
