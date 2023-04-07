use std::path::{PathBuf, Path};

use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};

pub struct FluxHitset {
    pub sounds: Vec<StaticSoundData>,
    pub index: usize,
}

impl FluxHitset {
    pub fn new() -> Self {
        Self {
            sounds: vec![],
            index: 0,
        }
    }

    pub fn load_from_path(&mut self, path: PathBuf) {
        self.sounds = vec![];
        for e in Path::new(&path).read_dir().unwrap() {
            let path = e.unwrap().path();
            if let Some(ext) = path.extension() {
                if ext == "mp3" || ext == "wav" || ext == "ogg" {
                    println!("Loaded hitsound {:?}", path);
                    let sound = StaticSoundData::from_file(path, StaticSoundSettings::default());
                    self.sounds.push(sound.expect("Failed to open hitset"));
                }
            }
        }
    }
}