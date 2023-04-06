use std::path::PathBuf;

use clap::{Parser, Subcommand, Args};
use map_creator::{FluxMap, convert::sspmv1::SSPM1};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArguments {
    /// where to save the map file - if bulk then it is treated as a folder
    #[arg(short,long)]
    out_path : PathBuf,
    #[command(subcommand)]
    command: Commands
}
#[derive(Subcommand)]
enum Commands {
    /// create multiple maps from another format in a folder.
    Bulk(BulkConvert),
    /// create a map with provided data
    Single(SingleConvert)
}
#[derive(Args)]
struct SingleConvert {
    /// name of the artist
    #[arg(short,long)]
    artist : String,
    /// name of the song
    #[arg(short,long)]
    song_name : String,
    /// name of the mapper
    #[arg(short,long)]
    mapper : String,
    /// path to the map (SS only) file
    #[arg(short='k',long)]
    map_path : PathBuf,
    /// path to the audio file
    #[arg(short='j',long)]
    audio_path : PathBuf,
}
#[derive(Args)]
struct BulkConvert {
    /// folder to read from
    #[arg(short,long)]
    in_path : PathBuf,
}
fn main() {
    let gargs : CliArguments = CliArguments::parse();

    match gargs.command {
        Commands::Single(args) => {
            if !args.map_path.exists() {
                panic!("Map file does not exist!");
            }
            if !args.audio_path.exists() {
                panic!("audio file does not exist!");
            }
            let map_data = std::fs::read(&args.map_path).expect("Failed to read map file. SHOULD NOT HAPPEN???");
            let audio_data = std::fs::read(&args.audio_path).expect("Failed to read audio file. SHOULD NOT HAPPEN???");
            let mut m = FluxMap::new();
            m.add_metadata("mapper".to_string(), args.mapper.as_bytes().to_vec());
            m.add_metadata("song_name".to_string(), args.song_name.as_bytes().to_vec());
            m.add_metadata("artist".to_string(), args.artist.as_bytes().to_vec());
            m.add_music(audio_data);
            m.add_difficulty("default".to_string(), FluxMap::convert_ss_to_flux(&map_data));
            m.save(gargs.out_path);

        }
        Commands::Bulk(args) => {
            for files in std::fs::read_dir(args.in_path).expect("unable to read in_path") {
                if let Ok(entry) = files {
                    let fname = entry.file_name();
                    let fname = fname.to_string_lossy();
                    if fname.ends_with(".sspm") {
                        let fdata = std::fs::read(entry.path()).expect("unable to read file");
                        let flux:FluxMap = SSPM1::try_from(fdata).expect("unable to parse data").into();
                        flux.save(gargs.out_path.join(fname.replace(".sspm",".flux")));
                    }
                }
            }
        }
    }
}
