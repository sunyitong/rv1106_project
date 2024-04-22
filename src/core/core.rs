use std::thread;
use std::time::{Instant, Duration};
use crate::core::track_loader::*;
use crate::core::data_loader_container::*;
use crate::core::wave_container::*;
use crate::core::operator_rack::*;
use crate::core::data_output::*;

pub struct Core {
    track_number: usize,
    data_loader_container: DataLoaderContainer,
    wave_container: WaveContainer,
    operator_rack: OperatorRack,
}

impl Core {
    pub fn new (track_number:usize) -> Self {
        Core {
            track_number,
            data_loader_container: DataLoaderContainer::new(track_number),
            wave_container: WaveContainer::new(track_number),
            operator_rack: OperatorRack::new(),
        }
    }

    pub fn core_loop (&mut self) {
        // loop track_loader from DataLoaderContainer
        // loop wave_track from WaveContainer
        // loop OperatorRack
        // loop DataOutput
    }

    fn set_track_loader (&mut self, loader_index:usize, loader_type:&str) {
        self.data_loader_container.set_track_loader(loader_index, loader_type, &self.wave_container);
    }

    fn read_track_from_file(&mut self, loader_index:usize, file_path:&str) {
        if let TrackLoader::FileLoader(ref mut loader) = &mut self.data_loader_container.track_loader[loader_index] {
            loader.read_track_from_file(file_path).unwrap();
        } else {
            todo!();
        }
    }

    fn save_track_from_file () {
        todo!();
    }

    fn push_data_to_track () {
        todo!();
    }

}