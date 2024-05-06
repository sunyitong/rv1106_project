use std::time::{Duration, Instant};
#[cfg(windows)]
use device_query::{DeviceQuery, DeviceState, Keycode};

pub struct KeyManager {
    device_state: DeviceState,
    keys: Vec<Keycode>,
    last_key_check: Instant,
    fps: Duration,

}

impl KeyManager {
    pub fn new (fps:f32) -> Self {
        KeyManager{
            device_state: DeviceState::new(),
            keys: Vec::new(),
            last_key_check: Instant::now(),
            fps: Duration::from_secs_f32(1.0 / fps),
        }
    }

    pub fn get_key(&mut self) -> String {
        if self.last_key_check.elapsed() >= self.fps {
            self.last_key_check = Instant::now();
            self.keys = self.device_state.get_keys();
            if let Some(key) = self.keys.first() {
                match key {
                    Keycode::Up => "Up".to_string(),
                    Keycode::Down => "Down".to_string(),
                    Keycode::Left => "Left".to_string(),
                    Keycode::Right => "Right".to_string(),
                    _ => "None".to_string(),
                }
            } else {
                "None".to_string()
            }
        } else {
            "None".to_string()
        }
    }
}

