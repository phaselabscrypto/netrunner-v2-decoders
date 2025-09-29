use super::ScoreDecoder;
pub mod process_close_accounts;
pub mod process_deregister_ship;
pub mod process_harvest;
pub mod process_initial_deposit;
pub mod process_initialize;
pub mod process_partial_deposit;
pub mod process_rearm;
pub mod process_refeed;
pub mod process_refuel;
pub mod process_register_ship;
pub mod process_repair;
pub mod process_settle;
pub mod process_update_reward_rate;
pub mod process_withdraw_arms;
pub mod process_withdraw_food;
pub mod process_withdraw_fuel;
pub mod process_withdraw_ships;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum ScoreInstruction {
    ProcessCloseAccounts(process_close_accounts::ProcessCloseAccounts),
    ProcessDeregisterShip(process_deregister_ship::ProcessDeregisterShip),
    ProcessHarvest(process_harvest::ProcessHarvest),
    ProcessInitialDeposit(process_initial_deposit::ProcessInitialDeposit),
    ProcessInitialize(process_initialize::ProcessInitialize),
    ProcessPartialDeposit(process_partial_deposit::ProcessPartialDeposit),
    ProcessRearm(process_rearm::ProcessRearm),
    ProcessRefeed(process_refeed::ProcessRefeed),
    ProcessRefuel(process_refuel::ProcessRefuel),
    ProcessRegisterShip(process_register_ship::ProcessRegisterShip),
    ProcessRepair(process_repair::ProcessRepair),
    ProcessSettle(process_settle::ProcessSettle),
    ProcessUpdateRewardRate(process_update_reward_rate::ProcessUpdateRewardRate),
    ProcessWithdrawArms(process_withdraw_arms::ProcessWithdrawArms),
    ProcessWithdrawFood(process_withdraw_food::ProcessWithdrawFood),
    ProcessWithdrawFuel(process_withdraw_fuel::ProcessWithdrawFuel),
    ProcessWithdrawShips(process_withdraw_ships::ProcessWithdrawShips),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for ScoreDecoder {
    type InstructionType = ScoreInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            ScoreInstruction::ProcessCloseAccounts => process_close_accounts::ProcessCloseAccounts,
            ScoreInstruction::ProcessDeregisterShip => process_deregister_ship::ProcessDeregisterShip,
            ScoreInstruction::ProcessHarvest => process_harvest::ProcessHarvest,
            ScoreInstruction::ProcessInitialDeposit => process_initial_deposit::ProcessInitialDeposit,
            ScoreInstruction::ProcessInitialize => process_initialize::ProcessInitialize,
            ScoreInstruction::ProcessPartialDeposit => process_partial_deposit::ProcessPartialDeposit,
            ScoreInstruction::ProcessRearm => process_rearm::ProcessRearm,
            ScoreInstruction::ProcessRefeed => process_refeed::ProcessRefeed,
            ScoreInstruction::ProcessRefuel => process_refuel::ProcessRefuel,
            ScoreInstruction::ProcessRegisterShip => process_register_ship::ProcessRegisterShip,
            ScoreInstruction::ProcessRepair => process_repair::ProcessRepair,
            ScoreInstruction::ProcessSettle => process_settle::ProcessSettle,
            ScoreInstruction::ProcessUpdateRewardRate => process_update_reward_rate::ProcessUpdateRewardRate,
            ScoreInstruction::ProcessWithdrawArms => process_withdraw_arms::ProcessWithdrawArms,
            ScoreInstruction::ProcessWithdrawFood => process_withdraw_food::ProcessWithdrawFood,
            ScoreInstruction::ProcessWithdrawFuel => process_withdraw_fuel::ProcessWithdrawFuel,
            ScoreInstruction::ProcessWithdrawShips => process_withdraw_ships::ProcessWithdrawShips,
        )
    }
}
