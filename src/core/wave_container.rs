use std::cell::RefCell;
use std::rc::Rc;
use std::fs::{OpenOptions};
use std::io::{self, Write, BufWriter};

pub struct WaveContainer {
    pub data_input_port: Vec<Rc<RefCell<i32>>>,
    pub wave_track: Vec<Rc<RefCell<Vec<i32>>>>,
    pub data_output_port: Vec<Rc<RefCell<i32>>>,
    track_selection: usize,
    recording_flag: Vec<bool>,
    pointer: usize,
    in_out_flag_show: bool,
    in_flag: usize,
    out_flag: usize,
    in_out_buffer: Vec<i32>,
}

impl WaveContainer {
    pub fn new (track_number:usize) -> Self {
        let data_input_port= (0..track_number).map(|_| Rc::new(RefCell::new(0))).collect::<Vec<_>>();
        let wave_track = (0..track_number).map(|_| Rc::new(RefCell::new(Vec::new()))).collect::<Vec<_>>();
        let data_output_port = (0..track_number).map(|_| Rc::new(RefCell::new(0))).collect::<Vec<_>>();
        WaveContainer{
            data_input_port,
            wave_track,
            data_output_port,
            track_selection: 0,
            recording_flag: vec![false; track_number],
            pointer: 0,
            in_out_flag_show: false,
            in_flag: 0,
            out_flag: 0,
            in_out_buffer: Vec::new(),
        }
    }

    pub fn record () {
        todo!()
    }

    pub fn mark_in_flag (&mut self) {
        self.in_out_flag_show = true;
        self.in_flag = self.pointer;
    }

    pub fn mark_out_flag (&mut self) {
        self.in_out_flag_show =true;
        self.out_flag = self.pointer;
    }

    pub fn cancel_in_out_flag (&mut self) {
        self.in_out_flag_show = false;
    }

    pub fn copy_in_out_track (&mut self) {
        if self.in_out_flag_show {
            self.in_out_buffer = self.wave_track[self.track_selection].borrow()[self.in_flag..=self.out_flag].to_vec();
        }
    }

    pub fn paste_in_out_track (&mut self) {
        let mut track_to_paste = self.wave_track[self.track_selection].borrow_mut();
        let required_length = self.pointer + self.in_out_buffer.len();
        if track_to_paste.len() < required_length {
            track_to_paste.resize(required_length, 0);
        }
        track_to_paste.splice(self.pointer..self.pointer+self.in_out_buffer.len(), self.in_out_buffer.iter().cloned());
    }

    pub fn insert_in_out_track (&mut self) {
        let mut track_to_paste = self.wave_track[self.track_selection].borrow_mut();
        track_to_paste.splice(self.pointer..self.pointer, self.in_out_buffer.iter().cloned());
    }

    pub fn select_track (&mut self, track_index:usize) {
        self.track_selection = track_index;
    }

    pub fn save_track_to_file (&self, track_index:usize, file_path: &str) -> io::Result<()> {
        let file = OpenOptions::new().write(true).create(true).open(file_path)?;
        let mut writer = BufWriter::new(file);
        for &value in self.wave_track[track_index].borrow().iter() {
            writer.write_all(&value.to_le_bytes())?;
        }
        writer.flush()?;
        Ok(())
    }

    pub fn info (&self) {
        println!("Wave Container Info:");
        for (index, item) in self.wave_track.iter().enumerate() {
            println!("{} | {:?}", index, item.borrow());
        }
    }
}