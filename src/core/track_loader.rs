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
pub enum Sensor {
    Simulator,
    None,
}

#[derive(Clone)]
pub struct TrackSensorReader{
    linked_track: Rc<RefCell<Vec<i32>>>,
    sensor: Sensor,
}

impl TrackSensorReader {
    pub fn new (linked_track: Rc<RefCell<Vec<i32>>>, sensor: Sensor) -> Self {
        TrackSensorReader{
            linked_track,
            sensor,
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
    linked_track: Rc<RefCell<Vec<i32>>>,
    wave_type:WaveGenerateType,
    amplitude:f32,
    wavelength:usize,
    sample_rate: f32,
    phase: f32,
    duty: f32,
    y_shift:i32,
    is_push_data:bool,
    pub wave_date_buffer: Vec<i32>,
}

impl TrackWaveGenerator {
    pub fn new (wave_type: WaveGenerateType,
                linked_track: Rc<RefCell<Vec<i32>>>,
                ) -> Self {
        
        TrackWaveGenerator{
            linked_track,
            wave_type,
            amplitude: 10.0,
            wavelength: 30,
            sample_rate: 30.0,
            phase: 0.0,
            duty: 0.5,
            y_shift: 0,
            is_push_data:false,
            wave_date_buffer: vec![0],
        }
    }

    pub fn generate_wave(&mut self) {
        let mut wave: Vec<i32> = Vec::with_capacity(self.wavelength);
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
        self.wave_date_buffer = wave;
    }

    fn generate_sine_wave(&self, index: usize) -> i32 {
        let frequency = self.sample_rate / self.wavelength as f32;
        let x = self.phase + 2.0 * PI * frequency * index as f32 / self.sample_rate;
        self.wave_value_transform(x.sin())
    }


    fn generate_triangle_wave(&self, index: usize) -> i32 {
        let frequency = self.sample_rate / self.wavelength as f32;
        let angular_frequency = 2.0 * PI * frequency;
        let x = self.phase + angular_frequency * index as f32 / self.sample_rate;
        
        let x = x % (2.0 * PI);
        
        if x < PI {
            self.wave_value_transform(2.0 * x / PI - 1.0)
        } else {
            self.wave_value_transform(1.0 - 2.0 * (x - PI) / PI)
        }
    }

    fn generate_sawtooth_wave(&self, index: usize) -> i32 {
        let frequency = self.sample_rate / self.wavelength as f32;
        let angular_frequency = 2.0 * PI * frequency;
        let x = self.phase + angular_frequency * index as f32 / self.sample_rate;
        self.wave_value_transform(2.0 * (x / (2.0 * PI) - (x / (2.0 * PI)).floor()) - 1.0)
    }

    fn generate_noise(&self) -> i32 {
        let x = rand::thread_rng().gen_range(-1.0..1.0);
        self.wave_value_transform(x)
    }

    fn generate_rectangular_wave(&self, index: usize, duty_cycle: f32) -> i32 {
        let period = self.wavelength;
        let duty_cycle_samples = (period as f32 * duty_cycle) as usize;

        if index % period < duty_cycle_samples {
            self.wave_value_transform(1.0)
        } else {
            self.wave_value_transform(-1.0)
        }
    }
    
    fn wave_value_transform (&self, value: f32) -> i32 {
        (value * self.amplitude).round() as i32 + self.y_shift
    }
    
    pub fn set_amplitude(&mut self, amplitude:i32) {
        self.amplitude = amplitude as f32;
        self.generate_wave();
    }
    
    pub fn set_wavelength(&mut self, wavelength:usize) {
        self.wavelength = wavelength;
        self.generate_wave();
    }
    
    pub fn set_phase(&mut self, phase: i32) {
        self.phase = phase as f32;
        self.generate_wave();
    }
    
    pub fn set_duty(&mut self, duty: f32) {
        let mut duty = duty;
        if duty < 0.0 {
            duty = 0.0;
        } else if duty > 1.0 {
            duty = 1.0;
        }
        self.duty = duty;
        self.generate_wave();
    }
    
    pub fn set_y_shift(&mut self, y_shift:i32) {
        self.y_shift = y_shift;
        self.generate_wave();
        
    }

    pub fn get_wave_value(&self, time: usize) -> i32 {
        let wave_data = &self.wave_date_buffer;
        let time = time % wave_data.len();
        wave_data[time]
    }
    
    pub fn push_value_to_track(&mut self, time:usize) {
        let mut linked_track = self.linked_track.borrow_mut();
        linked_track[time] = self.get_wave_value(time);
    }
}