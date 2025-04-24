use carbon_core::{borsh, CarbonDeserialize};
use mpl_bubblegum::types::UseMethod as BubblegumUseMethod;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}

impl From<UseMethod> for BubblegumUseMethod {
    fn from(value: UseMethod) -> Self {
        match value {
            UseMethod::Burn => BubblegumUseMethod::Burn,
            UseMethod::Multiple => BubblegumUseMethod::Multiple,
            UseMethod::Single => BubblegumUseMethod::Single,
        }
    }
}
