use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x56353b4cd92647d5")]
pub struct WithdrawOffchainReward {
    pub amount: u64,
}

pub struct WithdrawOffchainRewardInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub admin_group: solana_sdk::pubkey::Pubkey,
    pub pool_id: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub receiver_token_account: solana_sdk::pubkey::Pubkey,
    pub reward_vault_token_account: solana_sdk::pubkey::Pubkey,
    pub reward_config: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawOffchainReward {
    type ArrangedAccounts = WithdrawOffchainRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let admin_group = accounts.get(1)?;
        let pool_id = accounts.get(2)?;
        let token_mint = accounts.get(3)?;
        let receiver_token_account = accounts.get(4)?;
        let reward_vault_token_account = accounts.get(5)?;
        let reward_config = accounts.get(6)?;
        let token_program = accounts.get(7)?;
        let associated_token_program = accounts.get(8)?;

        Some(WithdrawOffchainRewardInstructionAccounts {
            authority: authority.pubkey,
            admin_group: admin_group.pubkey,
            pool_id: pool_id.pubkey,
            token_mint: token_mint.pubkey,
            receiver_token_account: receiver_token_account.pubkey,
            reward_vault_token_account: reward_vault_token_account.pubkey,
            reward_config: reward_config.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}
