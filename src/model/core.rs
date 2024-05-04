use crate::model::track_loader::*;
use crate::model::data_loader_container::*;
use crate::model::wave_container::*;
use crate::model::operator_rack::*;
use crate::model::data_output::*;

pub struct Core {
    track_number: usize,
    pub data_loader_container: DataLoaderContainer,
    pub wave_container: WaveContainer,
    pub operator_rack: OperatorRack,
    pub data_output: DataOutput,
}

impl Core {
    pub fn new (track_number:usize) -> Self {
        Core {
            track_number,
            data_loader_container: DataLoaderContainer::new(track_number),
            wave_container: WaveContainer::new(track_number),
            operator_rack: OperatorRack::new(track_number),
            data_output: DataOutput::new(),
        }
    }

    pub fn core_loop (&mut self) {
        // loop track_loader from DataLoaderContainer
        self.data_loader_container.loop_update_track_loader_container(5);
        // loop wave_track from WaveContainer
        // loop OperatorRack
        // loop DataOutput
    }
    

    pub fn set_track_loader (&mut self, loader_index:usize, loader_type:&str, wave_generate_type: WaveGenerateType) {
        self.data_loader_container.set_track_loader(loader_index, loader_type, wave_generate_type, &self.wave_container);
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