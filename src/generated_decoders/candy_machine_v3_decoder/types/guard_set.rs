
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct GuardSet {
    pub bot_tax: Option<BotTax>,
    pub sol_payment: Option<SolPayment>,
    pub token_payment: Option<TokenPayment>,
    pub start_date: Option<StartDate>,
    pub third_party_signer: Option<ThirdPartySigner>,
    pub token_gate: Option<TokenGate>,
    pub gatekeeper: Option<Gatekeeper>,
    pub end_date: Option<EndDate>,
    pub allow_list: Option<AllowList>,
    pub mint_limit: Option<MintLimit>,
    pub nft_payment: Option<NftPayment>,
    pub redeemed_amount: Option<RedeemedAmount>,
    pub address_gate: Option<AddressGate>,
    pub nft_gate: Option<NftGate>,
    pub nft_burn: Option<NftBurn>,
    pub token_burn: Option<TokenBurn>,
    pub freeze_sol_payment: Option<FreezeSolPayment>,
    pub freeze_token_payment: Option<FreezeTokenPayment>,
    pub program_gate: Option<ProgramGate>,
    pub allocation: Option<Allocation>,
    pub token2022_payment: Option<Token2022Payment>,
}
