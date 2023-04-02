use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArguments {
    #[arg(short,long)]
    artist : String,
    #[arg(short,long)]
    song_name : String,
    #[arg(short,long)]
    mapper : String,
    #[arg(short='k',long)]
    map_path : PathBuf,
    #[arg(short='j',long)]
    mp3_path : PathBuf,
    #[arg(short,long)]
    out_path : PathBuf,
}

fn main() {
    let args : CliArguments = CliArguments::parse();
    if !args.map_path.exists() {
        panic!("Map file does not exist!");
    }
    if !args.mp3_path.exists() {
        panic!("MP3 file does not exist!");
    }
    let map_data = std::fs::read(&args.map_path).expect("Failed to read map file. SHOULD NOT HAPPEN???");
    let mp3_data = std::fs::read(&args.mp3_path).expect("Failed to read mp3 file. SHOULD NOT HAPPEN???");
    let mut flm_data = Vec::<u8>::with_capacity(mp3_data.len() + map_data.len() + args.artist.len() + args.song_name.len() + args.mapper.len() + 48); //48 is just to be safe (doesn't increase file size)
    flm_data.extend((args.artist.len() as u16).to_be_bytes());
    flm_data.extend(args.artist.as_bytes());
    flm_data.extend((args.song_name.len() as u16).to_be_bytes());
    flm_data.extend(args.song_name.as_bytes());
    flm_data.extend((args.mapper.len() as u16).to_be_bytes());
    flm_data.extend(args.mapper.as_bytes());
    flm_data.extend((map_data.len() as u32).to_be_bytes());
    flm_data.extend(map_data);
    flm_data.extend(mp3_data);
    std::fs::write(&args.out_path, flm_data).expect("Failed to write flm file. SHOULD NOT HAPPEN???");
}
