use std::cell::RefCell;
use std::rc::Rc;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, BufReader, BufWriter};

#[derive(Clone)]
pub enum TrackLoader {
    FileLoader(TrackFileLoader),
    SensorReader(TrackSensorReader),
    WaveGenerator(TrackWaveGenerator),
    None,
}

#[derive(Clone)]
pub struct TrackFileLoader {
    linked_track: Rc<RefCell<Vec<i32>>>,
}

impl TrackFileLoader {
    pub fn new (linked_track: Rc<RefCell<Vec<i32>>>) -> Self {
        TrackFileLoader{
            linked_track,
        }
    }

    pub fn save_track_to_file(&self, file_path: &str) -> io::Result<()> {
        let file = OpenOptions::new().write(true).create(true).open(file_path)?;
        let mut writer = BufWriter::new(file);
        for &value in self.linked_track.borrow().iter() {
            writer.write_all(&value.to_le_bytes())?;
        }
        writer.flush()?;
        Ok(())
    }

    pub fn read_track_from_file(&mut self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut data = Vec::new();
        let mut buffer = [0u8; 4];
        while let Ok(_) = reader.read_exact(&mut buffer) {
            let value = i32::from_le_bytes(buffer);
            data.push(value);
        }
        *self.linked_track.borrow_mut() = data;
        Ok(())
    }
}

#[derive(Clone)]
pub struct TrackSensorReader{
    linked_track: Rc<RefCell<Vec<i32>>>,
}

impl TrackSensorReader {
    pub fn new (linked_track: Rc<RefCell<Vec<i32>>>) -> Self {
        TrackSensorReader{
            linked_track,
        }
    }
}

#[derive(Clone)]
pub struct TrackWaveGenerator{
    linked_track: Rc<RefCell<Vec<i32>>>,
}

impl TrackWaveGenerator {
    pub fn new (linked_track: Rc<RefCell<Vec<i32>>>) -> Self {
        TrackWaveGenerator{
            linked_track,
        }
    }
}