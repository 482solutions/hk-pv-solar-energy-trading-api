use codec::{Decode, Encode};
use substrate_api_client::AccountId;

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode)]
pub struct PalletUniquesItemDetails {
    pub owner: AccountId,
    pub approved: Option<AccountId>,
    pub isFrozen: bool,
    pub deposit: u128,
}

pub struct PalletUniquesItemMetadata {
    pub deposit: u128,
    pub data: String,
    pub isFrozen: bool,
}
