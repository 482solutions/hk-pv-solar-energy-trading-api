use crate::models::westmint::{PalletUniquesItemDetails, PalletUniquesItemMetadata};
use sp_application_crypto::Ss58Codec;
use substrate_api_client::rpc::ws_client::WsRpcClient;
use substrate_api_client::{Api, PlainTipExtrinsicParams};

const API_URL_WSS: &str = "wss://westmint-rpc.dwellir.com";

pub struct WestmintApi();

impl WestmintApi {
    pub fn get_nft_details_all(collection_id: u32) -> Vec<PalletUniquesItemDetails> {
        // let client = WsRpcClient::new(API_URL_WSS);
        // let api = Api::<sp_core::sr25519::Pair, _, PlainTipExtrinsicParams>::new(client).unwrap();

        // let result: PalletUniquesItemDetails = api
        //     .get_storage_double_map("Uniques", "Asset", 482, 650, None)
        //     .unwrap()
        //     .unwrap();
        // println!("NFT info : {:?}", result);
        // println!(
        //     "Owner addr ss58 : {}",
        //     Ss58Codec::to_ss58check(&result.owner)
        // );

        vec![]
    }

    pub fn get_nft_details_by_id(collection_id: u32, item_id: u32) -> PalletUniquesItemDetails {
        unimplemented!()
    }

    pub fn get_nft_metadata_all(collection_id: u32) -> Vec<PalletUniquesItemMetadata> {
        unimplemented!()
    }

    pub fn get_nft_metadata_by_id(collection_id: u32, item_id: u32) -> PalletUniquesItemMetadata {
        unimplemented!()
    }
}
