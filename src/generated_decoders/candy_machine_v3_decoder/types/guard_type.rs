use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum GuardType {
    BotTax,
    SolPayment,
    TokenPayment,
    StartDate,
    ThirdPartySigner,
    TokenGate,
    Gatekeeper,
    EndDate,
    AllowList,
    MintLimit,
    NftPayment,
    RedeemedAmount,
    AddressGate,
    NftGate,
    NftBurn,
    TokenBurn,
    FreezeSolPayment,
    FreezeTokenPayment,
    ProgramGate,
    Allocation,
    Token2022Payment,
}
