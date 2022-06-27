use rocket::{
    http::Status,
    serde::json::{Json, Value},
    State,
};

use crate::{models::energy::EnergyNFT, storage::FilesStorageBackend};

#[get("/")]
pub async fn get_nfts_for_sale(backend: &State<FilesStorageBackend>) -> (Status, Value) {
    super::generic_response(backend.get_nfts_for_sale().await)
}

#[post("/", format = "json", data = "<nft_item>")]
pub async fn sell_nft(
    nft_item: Json<EnergyNFT>,
    backend: &State<FilesStorageBackend>,
) -> (Status, Value) {
    super::generic_response(
        backend
            .sell_nft(nft_item.collection_id, nft_item.item_id)
            .await,
    )
}
