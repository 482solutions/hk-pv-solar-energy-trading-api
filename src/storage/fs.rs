use crate::blockchain::westmint::WestmintApi;
use crate::error::Error;
use crate::models::energy::{EnergyNFT, EnergyNFTDTO};

use serde_json;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Once;
use std::thread;
use std::time;

static START: Once = Once::new();

// const STORAGE_FOLDER_BASE: &str = "data";
const STORAGE_FOLDER_FETCHED: &str = "data/fetched";
const STORAGE_FOLDER_RPOCESSED: &str = "data/processed";

#[derive(Clone)]
pub struct FilesStorageBackend {}

impl FilesStorageBackend {
    pub fn new() -> FilesStorageBackend {
        START.call_once(|| {
            thread::spawn(move || {
                FilesStorageBackend::start_data_loading();
            });
        });

        FilesStorageBackend {}
    }

    #[tokio::main]
    pub async fn start_data_loading() {
        let interval = time::Duration::from_secs(600);
        loop {
            match FilesStorageBackend::load_nft_data().await {
                Ok(_) => {}
                Err(e) => {
                    println!("Error ocurred during data loading: {:?}\n Retrying ...", e)
                }
            }
            thread::sleep(interval);
        }
    }

    pub async fn load_nft_data() -> Result<(), Error> {
        let collection_id: u32 = 482;
        let data = WestmintApi::get_nft_metadata_all(collection_id)
            .await
            .unwrap();
        let storage_data_pairs: Vec<EnergyNFT> = data
            .into_iter()
            .map(|item| EnergyNFT {
                storage_key: item.0,
                metadata: item.1.data,
            })
            .collect();

        // Create if not exists
        fs::create_dir_all(STORAGE_FOLDER_FETCHED)?;

        for item in storage_data_pairs {
            let data_file = format!("{}/{}", STORAGE_FOLDER_FETCHED, item.storage_key);
            let mut file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(data_file)
                .unwrap();

            writeln!(file, "{}", item.metadata)?;
        }

        Ok(())
    }

    pub async fn get_nfts_for_sale(&self) -> Result<Vec<EnergyNFTDTO>, Error> {
        let mut data: Vec<EnergyNFT> = Vec::new();
        for entry in
            fs::read_dir(Path::new(STORAGE_FOLDER_FETCHED)).expect("Unable to open directory")
        {
            let file = entry?.path();
            let storage_key = file.file_name().unwrap().to_str().unwrap().to_string();
            let metadata = fs::read_to_string(file).expect("Unable to read file with metadata");

            data.push(EnergyNFT {
                storage_key,
                metadata,
            });
        }

        let data_dto: Vec<EnergyNFTDTO> = data
            .into_iter()
            .map(|item| EnergyNFTDTO {
                storage_key: item.storage_key,
                metadata: serde_json::from_str(&item.metadata).unwrap_or_default(),
            })
            .collect();

        // Move processed files from fetched folder to processed
        fs::create_dir_all(STORAGE_FOLDER_RPOCESSED)?;
        for entry in fs::read_dir(STORAGE_FOLDER_FETCHED)? {
            let entry = entry?;
            fs::rename(
                entry.path(),
                format!(
                    "{}/{}",
                    STORAGE_FOLDER_RPOCESSED,
                    entry.file_name().to_str().unwrap()
                ),
            )?;
        }

        Ok(data_dto)
    }

    pub async fn sell_nft(&self, storage_key: String) -> Result<(), Error> {
        println!("Item {} was sold...", storage_key);
        Ok(())
    }
}
