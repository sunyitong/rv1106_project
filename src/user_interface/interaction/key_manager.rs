#[cfg(windows)]
use device_query::{DeviceQuery, DeviceState, Keycode};

pub struct KeyManager {
    device_state: DeviceState,
    keys: Vec<Keycode>,
}

impl KeyManager {
    pub fn new () -> Self {
        KeyManager{
            device_state: DeviceState::new(),
            keys: Vec::new(),
        }
    }
    
    pub fn get_keys (&mut self) {
        let new_keys = self.device_state.get_keys();
        
        if !new_keys.is_empty() {
            self.keys = new_keys;
            println!("{:?}", self.keys);
        }
    }
}

