use std::cell::RefCell;
use std::rc::Rc;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::f32::consts::PI;
use rand::Rng;

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

#[derive(Clone, Debug)]
pub enum WaveGenerateType {
    Rectangular(f32),
    Sine,
    Triangle,
    Sawtooth,
    Noise,
}

#[derive(Clone, Debug)]
pub struct TrackWaveGenerator{
    // linked_track_input_port: Rc<RefCell<i32>>,
    // linked_track: Rc<RefCell<Vec<i32>>>,
    // linked_track_output_port: Rc<RefCell<i32>>,
    wave_type:WaveGenerateType,
    amplitude:i32,
    wavelength:usize,
    sample_rate: f32,
    phase: f32,
    duty: f32,
    pub wave_date_buffer: Option<Vec<f32>>,
}

impl TrackWaveGenerator {
    // linked_track: Rc<RefCell<Vec<i32>>>,
    // linked_track_input_port: Rc<RefCell<i32>>,
    // linked_track_output_port:Rc<RefCell<i32>>
    pub fn new (wave_type: WaveGenerateType,
                amplitude:i32,
                wavelength:usize,
                sample_rate: f32,
                ) -> Self {
        TrackWaveGenerator{
            // linked_track_input_port,
            // linked_track,
            // linked_track_output_port,
            wave_type,
            amplitude,
            wavelength,
            sample_rate,
            phase: 0.0,
            duty: 0.5,
            wave_date_buffer: None,
        }
    }

    pub fn generate_wave(&mut self) {
        let mut wave: Vec<f32> = Vec::with_capacity(self.wavelength);
        for i in 0.. self.wavelength {
            let value = match self.wave_type{
                WaveGenerateType::Rectangular(duty_cycle) => {
                    self.duty = duty_cycle;
                    self.generate_rectangular_wave(i, duty_cycle)
                },
                WaveGenerateType::Sine => self.generate_sine_wave(i),
                WaveGenerateType::Triangle => self.generate_triangle_wave(i),
                WaveGenerateType::Sawtooth => self.generate_sawtooth_wave(i),
                WaveGenerateType::Noise => self.generate_noise(),
            };

            wave.push(value);
        }
        self.wave_date_buffer = Some(wave);
    }

    fn generate_sine_wave(&self, index: usize) -> f32 {
        let frequency = self.sample_rate / self.wavelength as f32;
        let x = self.phase + 2.0 * PI * index as f32 * frequency;
        x.sin()
    }

    fn generate_triangle_wave(&self, index: usize) -> f32 {
        let frequency = self.sample_rate / self.wavelength as f32;
        let x = self.phase + 2.0 * index as f32 * frequency;
        (2.0 * (x - 2.0 * (x / (2.0 * PI)).floor()).abs() / PI) - 1.0
    }

    fn generate_sawtooth_wave(&self, index: usize) -> f32 {
        let frequency = self.sample_rate / self.wavelength as f32;
        let x = self.phase + index as f32 * frequency;
        (2.0 * (x - (x / (2.0 * PI)).floor() * PI) / PI) - 1.0
    }

    fn generate_noise(&self) -> f32 {
        rand::thread_rng().gen_range(-1.0..1.0)
    }

    fn generate_rectangular_wave(&self, index: usize, duty_cycle: f32) -> f32 {
        let period = self.wavelength;
        let duty_cycle_samples = (period as f32 * duty_cycle) as usize;

        if index % period < duty_cycle_samples {
            1.0
        } else {
            -1.0
        }
    }

    fn get_value_at_time(&self, time: usize) -> Option<i32> {
        if let Some(wave_data) = &self.wave_date_buffer {
            if time < wave_data.len() {
                Some((wave_data[time] * self.amplitude as f32) as i32)
            } else {
                None
            }
        } else {
            None
        }
    }
}