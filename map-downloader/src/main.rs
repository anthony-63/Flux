use std::path::PathBuf;

use futures::{future::join_all, stream::FuturesUnordered, StreamExt};
use map_creator::{FluxMap, convert::sspmv1::SSPM1};
use mapdata::{DbRoot, MapRoot};
use reqwest::Client;
use tokio::sync::Semaphore;

mod mapdata;
const DB : &'static str = "https://cdn.soundspaceplus.dev/";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let db_root :DbRoot = client.get(format!("{DB}/index.json")).send().await.unwrap().json().await.unwrap();
    let download_folder = PathBuf::from("./flux-downloads");
    std::fs::create_dir_all(&download_folder).expect("unable to create downloads folder");
    let sema = Semaphore::new(16);
    let mut downloads = Vec::new();
    for map_data in db_root.values() {
        let id = map_data.get("id").expect("unable to get id");
        let download_file_to = download_folder.join(format!("{}.flux",id.as_str().unwrap()));
        // println!("{:?}",download_file_to);
        // return;
        if download_file_to.exists() {
            println!("{} already exists, skipping.",id);
            continue;
        }
        let map_data : MapRoot = serde_json::from_value(map_data.clone()).expect("unable to parse data");
        println!("map id : {}",map_data.id);
        downloads.push(save(map_data,download_file_to, client.clone(), &sema))
        
    }
    let _ = join_all(downloads).await;
}
async fn save(map_data:MapRoot,path_to:PathBuf, client:Client, sema : &Semaphore) {
    if let Ok(map_datad) = download(map_data.id.clone(), client, sema).await {
        // let mut sspmw = path_to.clone();
        // sspmw.set_extension("sspm");
        // std::fs::write(sspmw, &map_datad).expect("aa");

        let map_datan = SSPM1::try_from(map_datad);
        if let Ok(d) = map_datan {
            println!("got map data, converting..");
            // return;
            
            FluxMap::from(d.into()).save(path_to.clone());
            println!("saved {:?}",&path_to)
        } else {
            println!("unable to parse map data for {}",map_data.id);
        }

    }
}
async fn download(id:String, client:Client, sema : &Semaphore) -> Result<Vec<u8>,()> {
    let sem = sema.acquire().await;
    println!("downloading {id}");
    let map_file = format!("{DB}/maps/{id}.sspm");
    let map_data = client.get(map_file).send().await.unwrap().bytes().await.unwrap().to_vec();
    println!("done {id}");
    Ok(map_data)
}