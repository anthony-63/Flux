use std::path::PathBuf;

use futures::{future::join_all, stream::FuturesUnordered, StreamExt};
use map_creator::FluxMap;
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
    let mut downloads = FuturesUnordered::new();
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
    futures::join!(downloads.into_future());
}
async fn save(map_data:MapRoot,path_to:PathBuf, client:Client, sema : &Semaphore) {
    if let Ok((map_datad,music_data)) = download(map_data.id, map_data.music_format.unwrap_or("mp3".to_string()), client, sema).await {
        let mut sspmw = path_to.clone();
        sspmw.set_extension("sspm");
        std::fs::write(sspmw, &map_datad).expect("aa");

        let convert_map = map_creator::convert::sspmv1::convert_sspm1(map_datad);

        return;


        FluxMap {
            artist: "<UNKNOWN>".to_string(),
            map_data: convert_map,
            song_name: map_data.song,
            mapper: map_data.author.join(",").to_string(),
            music_data
        }.save(path_to.clone());
        println!("saved {:?}",&path_to)
    }
}
async fn download(id:String,format:String, client:Client, sema : &Semaphore) -> Result<(Vec<u8>,Vec<u8>),()> {
    let sem = sema.acquire().await;
    println!("downloading {id}");
    let map_file = format!("{DB}/maps/{id}.sspm");
    let music_file = format!("{DB}/maps/{id}.{format}");
    let map_data = client.get(map_file).send().await.unwrap().bytes().await.unwrap().to_vec();
    let music_data = client.get(music_file).send().await.unwrap().bytes().await.unwrap().to_vec();
    println!("done {id}");
    Ok((map_data,music_data))
}