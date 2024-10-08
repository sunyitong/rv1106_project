use crate::model::track_loader::*;
use crate::model::wave_container::*;

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
                            wave_generate_type: WaveGenerateType,
                            wave_container: &WaveContainer) {
        let loader = &mut self.track_loader[loader_index];
        let track = &wave_container.wave_track[loader_index];
        *loader = match loader_type {
            "file_loader" => {
                TrackLoader::FileLoader(TrackFileLoader::new(track.clone()))
            },
            // "sensor_reader" => ,
            "wave_generator" => {
                let mut loader = TrackLoader::WaveGenerator(TrackWaveGenerator::new(wave_generate_type, track.clone()));
                if let TrackLoader::WaveGenerator(wave_generator) = &mut loader {
                    wave_generator.generate_wave();
                }
                loader
            },
            _ => TrackLoader::None,
        }
    }
    
    pub fn loop_update_track_loader_container(&mut self, time:usize) {
        for i in &self.track_loader {
           match i {
               TrackLoader::FileLoader(i) => todo!(),
               TrackLoader::WaveGenerator(i) => {
                   println!("{}", i.get_wave_value(time));
                   println!("{:?}", i.wave_date_buffer);
               }
               _=> println!("other! loop_update_track_loader_container")
           }
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