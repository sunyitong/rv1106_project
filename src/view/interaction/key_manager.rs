use std::time::{Duration, Instant};
#[cfg(windows)]
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashMap;

pub struct KeyManager {
    device_state: DeviceState,
    key_timers: HashMap<Keycode, Instant>, // Track when each key was first pressed
}

impl KeyManager {
    pub fn new() -> Self {
        KeyManager {
            device_state: DeviceState::new(),
            key_timers: HashMap::new(),
        }
    }

    pub fn check_keys(&mut self) -> Vec<String> {
        let now = Instant::now();
        let keys = self.device_state.get_keys();
        let mut output = Vec::new();

        // Update key timers and decide which key events to output
        for key in &keys {
            // Check if the key is newly pressed
            if !self.key_timers.contains_key(key) {
                self.key_timers.insert(*key, now);
                output.push(self.get_key_label(key));  // Output key label on first press
            }
        }

        // Remove keys that are no longer pressed
        self.key_timers.retain(|&k, &mut v| keys.contains(&k));
        
        output
    }

    fn get_key_label(&self, key: &Keycode) -> String {
        match key {
            Keycode::Up => "Up".to_string(),
            Keycode::Down => "Down".to_string(),
            Keycode::Left => "Left".to_string(),
            Keycode::Right => "Right".to_string(),
            _ => "".to_string(),  // Ignore other keys
        }
    }
}
