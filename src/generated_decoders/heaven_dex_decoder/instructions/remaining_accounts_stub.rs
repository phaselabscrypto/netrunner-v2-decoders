use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x208f535e17223bef")]
pub struct RemainingAccountsStub {}

pub struct RemainingAccountsStubInstructionAccounts {
    pub create_protocol_config_remaining_accounts: solana_sdk::pubkey::Pubkey,
    pub kamino_lending_user_metadata: solana_sdk::pubkey::Pubkey,
    pub kamino_lending_init_obligation: solana_sdk::pubkey::Pubkey,
    pub kamino_lending_refresh_reserve: solana_sdk::pubkey::Pubkey,
    pub kamino_lending_refresh_obligation: solana_sdk::pubkey::Pubkey,
    pub refresh_msol_price_list: solana_sdk::pubkey::Pubkey,
    pub msol_liquid_staking: solana_sdk::pubkey::Pubkey,
    pub msol_liquid_staking_cpi: solana_sdk::pubkey::Pubkey,
    pub kamino_deposit_with_farm: solana_sdk::pubkey::Pubkey,
    pub kamino_deposit_with_farm_cpi: solana_sdk::pubkey::Pubkey,
    pub kamino_deposit_with_farm_client: solana_sdk::pubkey::Pubkey,
    pub kamino_borrow_with_farm: solana_sdk::pubkey::Pubkey,
    pub kamino_borrow_with_farm_cpi: solana_sdk::pubkey::Pubkey,
    pub kamino_borrow_with_farm_client: solana_sdk::pubkey::Pubkey,
    pub kamino_repay_with_farm: solana_sdk::pubkey::Pubkey,
    pub kamino_repay_with_farm_cpi: solana_sdk::pubkey::Pubkey,
    pub kamino_repay_with_farm_client: solana_sdk::pubkey::Pubkey,
    pub order_unstake_msol_client: solana_sdk::pubkey::Pubkey,
    pub order_unstake_msol_cpi: solana_sdk::pubkey::Pubkey,
    pub claim_msol_cpi: solana_sdk::pubkey::Pubkey,
    pub kamino_withdraw_with_farm: solana_sdk::pubkey::Pubkey,
    pub kamino_withdraw_with_farm_cpi: solana_sdk::pubkey::Pubkey,
    pub kamino_withdraw_with_farm_client: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemainingAccountsStub {
    type ArrangedAccounts = RemainingAccountsStubInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let create_protocol_config_remaining_accounts = accounts.get(0)?;
        let kamino_lending_user_metadata = accounts.get(1)?;
        let kamino_lending_init_obligation = accounts.get(2)?;
        let kamino_lending_refresh_reserve = accounts.get(3)?;
        let kamino_lending_refresh_obligation = accounts.get(4)?;
        let refresh_msol_price_list = accounts.get(5)?;
        let msol_liquid_staking = accounts.get(6)?;
        let msol_liquid_staking_cpi = accounts.get(7)?;
        let kamino_deposit_with_farm = accounts.get(8)?;
        let kamino_deposit_with_farm_cpi = accounts.get(9)?;
        let kamino_deposit_with_farm_client = accounts.get(10)?;
        let kamino_borrow_with_farm = accounts.get(11)?;
        let kamino_borrow_with_farm_cpi = accounts.get(12)?;
        let kamino_borrow_with_farm_client = accounts.get(13)?;
        let kamino_repay_with_farm = accounts.get(14)?;
        let kamino_repay_with_farm_cpi = accounts.get(15)?;
        let kamino_repay_with_farm_client = accounts.get(16)?;
        let order_unstake_msol_client = accounts.get(17)?;
        let order_unstake_msol_cpi = accounts.get(18)?;
        let claim_msol_cpi = accounts.get(19)?;
        let kamino_withdraw_with_farm = accounts.get(20)?;
        let kamino_withdraw_with_farm_cpi = accounts.get(21)?;
        let kamino_withdraw_with_farm_client = accounts.get(22)?;

        Some(RemainingAccountsStubInstructionAccounts {
            create_protocol_config_remaining_accounts: create_protocol_config_remaining_accounts
                .pubkey,
            kamino_lending_user_metadata: kamino_lending_user_metadata.pubkey,
            kamino_lending_init_obligation: kamino_lending_init_obligation.pubkey,
            kamino_lending_refresh_reserve: kamino_lending_refresh_reserve.pubkey,
            kamino_lending_refresh_obligation: kamino_lending_refresh_obligation.pubkey,
            refresh_msol_price_list: refresh_msol_price_list.pubkey,
            msol_liquid_staking: msol_liquid_staking.pubkey,
            msol_liquid_staking_cpi: msol_liquid_staking_cpi.pubkey,
            kamino_deposit_with_farm: kamino_deposit_with_farm.pubkey,
            kamino_deposit_with_farm_cpi: kamino_deposit_with_farm_cpi.pubkey,
            kamino_deposit_with_farm_client: kamino_deposit_with_farm_client.pubkey,
            kamino_borrow_with_farm: kamino_borrow_with_farm.pubkey,
            kamino_borrow_with_farm_cpi: kamino_borrow_with_farm_cpi.pubkey,
            kamino_borrow_with_farm_client: kamino_borrow_with_farm_client.pubkey,
            kamino_repay_with_farm: kamino_repay_with_farm.pubkey,
            kamino_repay_with_farm_cpi: kamino_repay_with_farm_cpi.pubkey,
            kamino_repay_with_farm_client: kamino_repay_with_farm_client.pubkey,
            order_unstake_msol_client: order_unstake_msol_client.pubkey,
            order_unstake_msol_cpi: order_unstake_msol_cpi.pubkey,
            claim_msol_cpi: claim_msol_cpi.pubkey,
            kamino_withdraw_with_farm: kamino_withdraw_with_farm.pubkey,
            kamino_withdraw_with_farm_cpi: kamino_withdraw_with_farm_cpi.pubkey,
            kamino_withdraw_with_farm_client: kamino_withdraw_with_farm_client.pubkey,
        })
    }
}
