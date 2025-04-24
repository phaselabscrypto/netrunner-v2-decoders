use carbon_core::{borsh, CarbonDeserialize};
use mpl_bubblegum::types::TokenProgramVersion as BubblegumTokenProgramVersion;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum TokenProgramVersion {
    Original,
    Token2022,
}

impl From<TokenProgramVersion> for BubblegumTokenProgramVersion {
    fn from(value: TokenProgramVersion) -> Self {
        match value {
            TokenProgramVersion::Original => BubblegumTokenProgramVersion::Original,
            TokenProgramVersion::Token2022 => BubblegumTokenProgramVersion::Token2022,
        }
    }
}
