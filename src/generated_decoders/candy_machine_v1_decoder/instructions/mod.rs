



use super::NftCandyMachineDecoder;
pub mod mint_nft;
pub mod update_candy_machine;
pub mod initialize_config;
pub mod add_config_lines;
pub mod initialize_candy_machine;
pub mod update_authority;
pub mod withdraw_funds;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum NftCandyMachineInstruction {
    MintNft(mint_nft::MintNft),
    UpdateCandyMachine(update_candy_machine::UpdateCandyMachine),
    InitializeConfig(initialize_config::InitializeConfig),
    AddConfigLines(add_config_lines::AddConfigLines),
    InitializeCandyMachine(initialize_candy_machine::InitializeCandyMachine),
    UpdateAuthority(update_authority::UpdateAuthority),
    WithdrawFunds(withdraw_funds::WithdrawFunds),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for NftCandyMachineDecoder {
    type InstructionType = NftCandyMachineInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            NftCandyMachineInstruction::MintNft => mint_nft::MintNft,
            NftCandyMachineInstruction::UpdateCandyMachine => update_candy_machine::UpdateCandyMachine,
            NftCandyMachineInstruction::InitializeConfig => initialize_config::InitializeConfig,
            NftCandyMachineInstruction::AddConfigLines => add_config_lines::AddConfigLines,
            NftCandyMachineInstruction::InitializeCandyMachine => initialize_candy_machine::InitializeCandyMachine,
            NftCandyMachineInstruction::UpdateAuthority => update_authority::UpdateAuthority,
            NftCandyMachineInstruction::WithdrawFunds => withdraw_funds::WithdrawFunds,
        )
    }
}