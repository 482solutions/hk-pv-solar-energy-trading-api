use rocket::{
    http::Status,
    serde::json::{Json, Value},
    State,
};

use crate::{models::energy::EnergyNFT, storage::fs::FilesStorageBackend};

#[get("/for_sale")]
pub async fn nfts_for_sale(backend: &State<FilesStorageBackend>) -> (Status, Value) {
    super::generic_response(backend.get_nfts_for_sale().await)
}

#[post("/sell", format = "json", data = "<nft_item>")]
pub async fn nft_sell(
    nft_item: Json<EnergyNFT>,
    backend: &State<FilesStorageBackend>,
) -> (Status, Value) {
    super::generic_response(
        backend
            .sell_nft(nft_item.0.collection_id, nft_item.0.item_id)
            .await,
    )
}
