#[cfg(windows)]
use device_query::{DeviceQuery, DeviceState, Keycode};

pub struct KeyManager {
    device_state: DeviceState,
    pub keys: Vec<Keycode>,
}

impl KeyManager {
    pub fn new () -> Self {
        KeyManager{
            device_state: DeviceState::new(),
            keys: Vec::new(),
        }
    }
    
    pub fn get_keys (&mut self) {
        self.keys = self.device_state.get_keys();
    }
}

