use std::{path::PathBuf, time::Duration};

use futures::future::join_all;
use flux_map::{convert::sspm::SSPM, FluxMap};
use reqwest::Client;
use serde_json::Value;
use tokio::{sync::Semaphore, time::timeout};
const DB: &'static str = "https://cdn.soundspaceplus.dev/";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let db_root: serde_json::Map<String, Value> = client
        .get(format!("{DB}/index.json"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let download_folder = PathBuf::from("./flux-downloads");

    std::fs::create_dir_all(&download_folder).expect("unable to create downloads folder");

    // using semaphores to limit the amount of concurrent downloads
    let sema = Semaphore::new(16);
    let mut downloads = Vec::new();

    for map_data in db_root.values() {
        let id = if let Some(id) = map_data.get("id") {
            id
        } else {
            continue;
        };
        let id = id.as_str().unwrap().to_string();

        let download_file_to = download_folder.join(format!("{}.flux", &id));

        if download_file_to.exists() {
            println!("{} already exists, skipping.", id);
            continue;
        }

        downloads.push(save(
            id,
            download_file_to,
            client.clone(),
            &sema,
        ))
    }
    let _ = join_all(downloads).await;
}
async fn save(id: String, path_to: PathBuf, client: Client, sema: &Semaphore) {
    if let Ok(map_bytes) =
        download(id.clone(), client, sema).await
    {
        if let Err(e) = map_bytes {
            println!("failed to download {:?} {:?}", id, e);
            return;
        }

        let map_bytes = map_bytes.unwrap();
        println!("downloaded {:?} ({:?} bytes)", id, map_bytes.len());
        let sspm_map = SSPM::try_from(&map_bytes[..]);

        match sspm_map {
            Ok(map) => {
                FluxMap::from(map.into()).save(path_to.clone());
                println!("saved {:?}", &path_to)
            }
            Err(e) => {
                println!("failed to parse {:?}. why = {:?}", id, e)
            }
        }
    } else {
        println!("{:?} took too long to download..", id)
    }
}
async fn download(id: String, client: Client, sema: &Semaphore) -> Result<reqwest::Result<bytes::Bytes>,()> {
    let _a = sema.acquire().await;
    let url = format!("{DB}/maps/{id}.sspm");
    timeout(Duration::from_secs(60), async {

        let d = client.get(url).send().await?;
        let d= d.bytes().await;
        // fake error
        d
    }).await.map_err(|_| ())
}
