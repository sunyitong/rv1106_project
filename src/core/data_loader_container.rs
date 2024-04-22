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

    pub fn set_track_loader(&mut self, 
                            loader_index: usize, 
                            loader_type: &str, 
                            wave_container: &WaveContainer) {
        let loader = &mut self.track_loader[loader_index];
        let track_input_port = &wave_container.data_input_port[loader_index];
        let track = &wave_container.wave_track[loader_index];
        let track_out_port = &wave_container.data_output_port[loader_index];
        *loader = match loader_type {
            "file_loader" => TrackLoader::FileLoader(TrackFileLoader::new(track.clone())),
            // "sensor_reader" => ,
            // "wave_generator" => TrackLoader::WaveGenerator(TrackWaveGenerator::new(track.clone(),track_input_port.clone(), track_out_port.clone())),
            _ => TrackLoader::None,
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