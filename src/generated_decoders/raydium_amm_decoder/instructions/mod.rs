



use super::RaydiumAmmDecoder;
pub mod initialize;
pub mod initialize2;
pub mod monitor_step;
pub mod deposit;
pub mod withdraw;
pub mod migrate_to_open_book;
pub mod set_params;
pub mod withdraw_pnl;
pub mod withdraw_srm;
pub mod swap_base_in;
pub mod pre_initialize;
pub mod swap_base_out;
pub mod simulate_info;
pub mod admin_cancel_orders;
pub mod create_config_account;
pub mod update_config_account;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum RaydiumAmmInstruction {
    Initialize(initialize::Initialize),
    Initialize2(initialize2::Initialize2),
    MonitorStep(monitor_step::MonitorStep),
    Deposit(deposit::Deposit),
    Withdraw(withdraw::Withdraw),
    MigrateToOpenBook(migrate_to_open_book::MigrateToOpenBook),
    SetParams(set_params::SetParams),
    WithdrawPnl(withdraw_pnl::WithdrawPnl),
    WithdrawSrm(withdraw_srm::WithdrawSrm),
    SwapBaseIn(swap_base_in::SwapBaseIn),
    PreInitialize(pre_initialize::PreInitialize),
    SwapBaseOut(swap_base_out::SwapBaseOut),
    SimulateInfo(simulate_info::SimulateInfo),
    AdminCancelOrders(admin_cancel_orders::AdminCancelOrders),
    CreateConfigAccount(create_config_account::CreateConfigAccount),
    UpdateConfigAccount(update_config_account::UpdateConfigAccount),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for RaydiumAmmDecoder {
    type InstructionType = RaydiumAmmInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            RaydiumAmmInstruction::Initialize => initialize::Initialize,
            RaydiumAmmInstruction::Initialize2 => initialize2::Initialize2,
            RaydiumAmmInstruction::MonitorStep => monitor_step::MonitorStep,
            RaydiumAmmInstruction::Deposit => deposit::Deposit,
            RaydiumAmmInstruction::Withdraw => withdraw::Withdraw,
            RaydiumAmmInstruction::MigrateToOpenBook => migrate_to_open_book::MigrateToOpenBook,
            RaydiumAmmInstruction::SetParams => set_params::SetParams,
            RaydiumAmmInstruction::WithdrawPnl => withdraw_pnl::WithdrawPnl,
            RaydiumAmmInstruction::WithdrawSrm => withdraw_srm::WithdrawSrm,
            RaydiumAmmInstruction::SwapBaseIn => swap_base_in::SwapBaseIn,
            RaydiumAmmInstruction::PreInitialize => pre_initialize::PreInitialize,
            RaydiumAmmInstruction::SwapBaseOut => swap_base_out::SwapBaseOut,
            RaydiumAmmInstruction::SimulateInfo => simulate_info::SimulateInfo,
            RaydiumAmmInstruction::AdminCancelOrders => admin_cancel_orders::AdminCancelOrders,
            RaydiumAmmInstruction::CreateConfigAccount => create_config_account::CreateConfigAccount,
            RaydiumAmmInstruction::UpdateConfigAccount => update_config_account::UpdateConfigAccount,
        )
    }
}