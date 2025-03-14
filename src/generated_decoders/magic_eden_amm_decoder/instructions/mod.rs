



use super::MmmDecoder;
pub mod create_pool;
pub mod update_pool;
pub mod update_allowlists;
pub mod sol_close_pool;
pub mod sol_deposit_buy;
pub mod sol_withdraw_buy;
pub mod sol_fulfill_buy;
pub mod sol_fulfill_sell;
pub mod withdraw_sell;
pub mod deposit_sell;
pub mod ocp_deposit_sell;
pub mod sol_ocp_fulfill_buy;
pub mod sol_ocp_fulfill_sell;
pub mod ocp_withdraw_sell;
pub mod mip1_deposit_sell;
pub mod mip1_withdraw_sell;
pub mod sol_mip1_fulfill_sell;
pub mod sol_mip1_fulfill_buy;
pub mod close_if_balance_invalid;
pub mod set_shared_escrow;
pub mod ext_deposit_sell;
pub mod sol_ext_fulfill_sell;
pub mod sol_ext_fulfill_buy;
pub mod ext_withdraw_sell;
pub mod mpl_core_deposit_sell;
pub mod mpl_core_withdraw_sell;
pub mod sol_mpl_core_fulfill_sell;
pub mod sol_mpl_core_fulfill_buy;
pub mod cnft_fulfill_buy;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum MmmInstruction {
    CreatePool(create_pool::CreatePool),
    UpdatePool(update_pool::UpdatePool),
    UpdateAllowlists(update_allowlists::UpdateAllowlists),
    SolClosePool(sol_close_pool::SolClosePool),
    SolDepositBuy(sol_deposit_buy::SolDepositBuy),
    SolWithdrawBuy(sol_withdraw_buy::SolWithdrawBuy),
    SolFulfillBuy(sol_fulfill_buy::SolFulfillBuy),
    SolFulfillSell(sol_fulfill_sell::SolFulfillSell),
    WithdrawSell(withdraw_sell::WithdrawSell),
    DepositSell(deposit_sell::DepositSell),
    OcpDepositSell(ocp_deposit_sell::OcpDepositSell),
    SolOcpFulfillBuy(sol_ocp_fulfill_buy::SolOcpFulfillBuy),
    SolOcpFulfillSell(sol_ocp_fulfill_sell::SolOcpFulfillSell),
    OcpWithdrawSell(ocp_withdraw_sell::OcpWithdrawSell),
    Mip1DepositSell(mip1_deposit_sell::Mip1DepositSell),
    Mip1WithdrawSell(mip1_withdraw_sell::Mip1WithdrawSell),
    SolMip1FulfillSell(sol_mip1_fulfill_sell::SolMip1FulfillSell),
    SolMip1FulfillBuy(sol_mip1_fulfill_buy::SolMip1FulfillBuy),
    CloseIfBalanceInvalid(close_if_balance_invalid::CloseIfBalanceInvalid),
    SetSharedEscrow(set_shared_escrow::SetSharedEscrow),
    ExtDepositSell(ext_deposit_sell::ExtDepositSell),
    SolExtFulfillSell(sol_ext_fulfill_sell::SolExtFulfillSell),
    SolExtFulfillBuy(sol_ext_fulfill_buy::SolExtFulfillBuy),
    ExtWithdrawSell(ext_withdraw_sell::ExtWithdrawSell),
    MplCoreDepositSell(mpl_core_deposit_sell::MplCoreDepositSell),
    MplCoreWithdrawSell(mpl_core_withdraw_sell::MplCoreWithdrawSell),
    SolMplCoreFulfillSell(sol_mpl_core_fulfill_sell::SolMplCoreFulfillSell),
    SolMplCoreFulfillBuy(sol_mpl_core_fulfill_buy::SolMplCoreFulfillBuy),
    CnftFulfillBuy(cnft_fulfill_buy::CnftFulfillBuy),
}

impl carbon_core::instruction::InstructionDecoder<'_> for MmmDecoder {
    type InstructionType = MmmInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            MmmInstruction::CreatePool => create_pool::CreatePool,
            MmmInstruction::UpdatePool => update_pool::UpdatePool,
            MmmInstruction::UpdateAllowlists => update_allowlists::UpdateAllowlists,
            MmmInstruction::SolClosePool => sol_close_pool::SolClosePool,
            MmmInstruction::SolDepositBuy => sol_deposit_buy::SolDepositBuy,
            MmmInstruction::SolWithdrawBuy => sol_withdraw_buy::SolWithdrawBuy,
            MmmInstruction::SolFulfillBuy => sol_fulfill_buy::SolFulfillBuy,
            MmmInstruction::SolFulfillSell => sol_fulfill_sell::SolFulfillSell,
            MmmInstruction::WithdrawSell => withdraw_sell::WithdrawSell,
            MmmInstruction::DepositSell => deposit_sell::DepositSell,
            MmmInstruction::OcpDepositSell => ocp_deposit_sell::OcpDepositSell,
            MmmInstruction::SolOcpFulfillBuy => sol_ocp_fulfill_buy::SolOcpFulfillBuy,
            MmmInstruction::SolOcpFulfillSell => sol_ocp_fulfill_sell::SolOcpFulfillSell,
            MmmInstruction::OcpWithdrawSell => ocp_withdraw_sell::OcpWithdrawSell,
            MmmInstruction::Mip1DepositSell => mip1_deposit_sell::Mip1DepositSell,
            MmmInstruction::Mip1WithdrawSell => mip1_withdraw_sell::Mip1WithdrawSell,
            MmmInstruction::SolMip1FulfillSell => sol_mip1_fulfill_sell::SolMip1FulfillSell,
            MmmInstruction::SolMip1FulfillBuy => sol_mip1_fulfill_buy::SolMip1FulfillBuy,
            MmmInstruction::CloseIfBalanceInvalid => close_if_balance_invalid::CloseIfBalanceInvalid,
            MmmInstruction::SetSharedEscrow => set_shared_escrow::SetSharedEscrow,
            MmmInstruction::ExtDepositSell => ext_deposit_sell::ExtDepositSell,
            MmmInstruction::SolExtFulfillSell => sol_ext_fulfill_sell::SolExtFulfillSell,
            MmmInstruction::SolExtFulfillBuy => sol_ext_fulfill_buy::SolExtFulfillBuy,
            MmmInstruction::ExtWithdrawSell => ext_withdraw_sell::ExtWithdrawSell,
            MmmInstruction::MplCoreDepositSell => mpl_core_deposit_sell::MplCoreDepositSell,
            MmmInstruction::MplCoreWithdrawSell => mpl_core_withdraw_sell::MplCoreWithdrawSell,
            MmmInstruction::SolMplCoreFulfillSell => sol_mpl_core_fulfill_sell::SolMplCoreFulfillSell,
            MmmInstruction::SolMplCoreFulfillBuy => sol_mpl_core_fulfill_buy::SolMplCoreFulfillBuy,
            MmmInstruction::CnftFulfillBuy => cnft_fulfill_buy::CnftFulfillBuy,
        )
    }
}