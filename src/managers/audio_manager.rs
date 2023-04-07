use std::io::Cursor;

use kira::{manager::{AudioManager, backend::cpal::CpalBackend, AudioManagerSettings}, tween::Tween, sound::static_sound::{StaticSoundData, StaticSoundSettings}, PlaybackRate};

use crate::{core::{maploader::FluxMap, config::FluxConfig}, sets::hitset::FluxHitset};

pub struct FluxAudioManager {
    song_manager: AudioManager,
    hitsound_manager: AudioManager,    
}

impl FluxAudioManager {
    pub fn new() -> Self {
        Self {
            song_manager: AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).expect("Failed to create song audio manager"),
            hitsound_manager: AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).expect("Failed to create hitsound audio manager"),
        }
    }

    pub fn reset(&mut self) {
        self.song_manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).expect("Failed to create song audio manager");
        self.hitsound_manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).expect("Failed to create hitsound audio manager");
    }

    pub fn set_song_volume(&mut self, vol: f64) {
        self.song_manager.main_track().set_volume(vol, Tween {
            ..Default::default()
        });
    }

    pub fn set_hitsound_volume(&mut self, vol: f64) {
        self.hitsound_manager.main_track().set_volume(vol, Tween {
            ..Default::default()
        });
    }

    pub fn play_song(&mut self, map: &FluxMap, config: &FluxConfig) {
        let cursor = Cursor::new(map.mp3_data.clone());
        let sound_data = StaticSoundData::from_cursor(cursor, StaticSoundSettings::default().playback_rate(PlaybackRate::Factor(config.audio.speed))).expect("Failed to create sound data");
        self.song_manager.play(sound_data.clone()).unwrap();
    }

    pub fn pause_song(&mut self) {
        self.song_manager.pause(Tween {
            ..Default::default()
        }).unwrap();
    }

    pub fn resume_song(&mut self) {
        self.song_manager.resume(Tween {
            ..Default::default()
        }).unwrap();
    }

    pub fn play_hitsound(&mut self, hitset: &FluxHitset) {
        self.song_manager.play(hitset.sounds[hitset.index].clone());
    }
}