mod view;
mod model;
mod controller;
mod const_parameter;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};
use view::display::display::*;
use rand::Rng;
use model::core::*;
use crate::model::operator_rack::{OperatorAdd, OperatorRack, Port};
use crate::model::track_loader::{TrackWaveGenerator, WaveGenerateType};
use crate::view::interaction::key_manager::*;

use std::env;
use std::path::PathBuf;
use crate::model::track_loader::WaveGenerateType::Noise;
use crate::view::view_main::{ViewContainer};
use log::{info};

fn main() {
    env_logger::init();
    info!("starting up info");
    
    let mut view_container = ViewContainer::new(30.0, 4);
    view_container.frame_init();
    loop{
        view_container.frame_start();
        view_container.frame_main();
        view_container.frame_end();
    }
}
