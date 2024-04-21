use std::cell::RefCell;
use std::rc::Rc;
use crate::core::track_loader::*;
use crate::core::wave_container::*;

pub struct DataLoaderContainer {
    pub track_loader: Vec<TrackLoader>,
}

impl DataLoaderContainer {
    pub fn new (track_number:usize) -> Self {
        DataLoaderContainer {
            track_loader: vec![TrackLoader::None; track_number],
        }
    }

    pub fn set_track_loader(&mut self, loader_index: usize, loader_type: &str, wave_container: &WaveContainer) {
        let loader = &mut self.track_loader[loader_index];
        let track = &wave_container.wave_track[loader_index];
        *loader = Self::create_loader(loader_type, track.clone());
    }

    pub fn create_loader(loader_type: &str, track: Rc<RefCell<Vec<i32>>>) -> TrackLoader {
        match loader_type {
            "file_loader" => TrackLoader::FileLoader(TrackFileLoader::new(track)),
            // "sensor_reader" => ,
            // "wave_generator" => ,
            _ => TrackLoader::FileLoader(TrackFileLoader::new(track)), // Default fallback
        }
    }

    pub fn info (&self) {
        println!("Data Loader Container Info:");
        for (index, item) in self.track_loader.iter().enumerate() {
            match item {
                TrackLoader::FileLoader(_track_file_loader) => println!("{} | FileLoader", index),
                TrackLoader::SensorReader(_track_sensor_loader) => println!("{} | SensorReader", index),
                TrackLoader::WaveGenerator(_track_wave_generator) => println!("{} |WaveGenerator", index),
                TrackLoader::None => println!("{} | None", index),
            }
        }
    }
}