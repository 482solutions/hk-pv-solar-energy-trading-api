use rocket::{http::Status, serde::json::Value, State};

use crate::storage::fs::FilesStorageBackend;

#[get("/for_sale")]
pub async fn nfts_for_sale(backend: &State<FilesStorageBackend>) -> (Status, Value) {
    super::generic_response(backend.get_nfts_for_sale().await)
}

#[post("/sell", format = "json", data = "<storage_key>")]
pub async fn nft_sell(
    storage_key: String,
    backend: &State<FilesStorageBackend>,
) -> (Status, Value) {
    super::generic_response(backend.sell_nft(storage_key).await)
}
