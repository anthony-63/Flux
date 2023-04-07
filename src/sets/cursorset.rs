use std::path::Path;
use std::path::PathBuf;

use nannou::wgpu;
use nannou::prelude::*;

pub struct FluxCursorset {
    pub textures: Vec<wgpu::Texture>,
    pub index: usize,
}

impl FluxCursorset {
    pub fn new() -> Self {
        Self {
            textures: vec![],
            index: 0,
        }
    }

    pub fn load_from_path(&mut self, app: &App, path: PathBuf) {
        self.textures = vec![];
        for e in Path::new(&path).read_dir().unwrap() {
            let path = e.unwrap().path();
            if let Some(ext) = path.extension() {
                if ext == "png" || ext == "jpg" || ext == "jpeg" {
                    println!("Loaded cursor image: {:?}", path);
                    let tex = wgpu::Texture::from_path(app, String::from(path.to_str().unwrap())).unwrap();
                    self.textures.push(tex);
                }
            }
        }
    }
}