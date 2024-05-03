mod user_interface;
mod core;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};
use user_interface::display::display::*;
use rand::Rng;
use core::core::*;
use crate::core::operator_rack::{OperatorAdd, OperatorRack, Port};
use crate::core::track_loader::{TrackWaveGenerator, WaveGenerateType};
use crate::user_interface::interaction::key_manager::*;

use std::env;
use std::path::PathBuf;
use device_query::Keycode::K;
use crate::core::track_loader::WaveGenerateType::Noise;

struct LoopManager {
    loop_start_time: Instant,
    fps: Duration,
    display:Display,
    core: Core,
    key_manager: KeyManager,
}

impl LoopManager {
    fn new (fps: f32) -> Self {
        LoopManager{
            loop_start_time: Instant::now(),
            fps: Duration::from_secs_f32(1.0 / fps),
            display: Display::new(480, 480, 480 * 4,4),
            core: Core::new(6),
            key_manager: KeyManager::new(),
        }
    }
    fn init(&mut self) {
        self.core.set_track_loader(0,"wave_generator", Noise);
    }
    
    fn run_once_debug (&self) {
        // let mut fake_linked_track = Rc::new(RefCell::new(vec![0;30]));
        // 
        // let mut wave_generator_1 = TrackWaveGenerator::new(WaveGenerateType::Triangle, fake_linked_track.clone());
        // wave_generator_1.generate_wave();
        // println!("{:?}", &wave_generator_1);
        // 
        // let mut wave_generator_2 = TrackWaveGenerator::new(WaveGenerateType::Sawtooth, fake_linked_track.clone());
        // wave_generator_2.generate_wave();
        // println!("{:?}", &wave_generator_2);
        // 
        // 
        // for i in 0..10 {
        //     wave_generator_2.push_value_to_track(i);
        //     println!("{:?}", fake_linked_track.borrow());
        // }

        let mut rack = OperatorRack::new(6);  // 创建包含6个端口的输入输出节点

        // 添加四个加法节点
        let add_node1 = Box::new(OperatorAdd::new());
        let add_node2 = Box::new(OperatorAdd::new());
        let add_node3 = Box::new(OperatorAdd::new());
        let add_node4 = Box::new(OperatorAdd::new());
        rack.add_node(2, add_node1);
        rack.add_node(3, add_node2);
        rack.add_node(4, add_node3);
        rack.add_node(5, add_node4);
        
        for i in 0..2 {
            rack.connect(0, i, 2, i);
        }
        
        rack.connect(0,3,3,1);
        rack.connect(2, 0, 3, 0);
        
        rack.connect(2, 0, 1, 1);
        
        rack.connect(3, 0, 1, 0);
        
        rack.connect(0, 0, 4, 0);
        
        rack.connect(4, 0, 5, 0);
        
        rack.connect(0, 4, 5, 1);
        
        rack.connect(5, 0, 1, 4);
        
        rack.connect(3,0,4,1);

        // 设置输入值
        if let Some(input_node) = rack.operators.get_mut(&0) {
            for i in 0..6 {
                if let Some(port) = input_node.get_output_port(&i) {
                    *port.borrow_mut() = Port { value: ((i + 1) * 10) as i32 };  // 为每个端口设置值: 10, 20, ..., 60
                }
            }
        }

        // 执行计算
        rack.compute();

        // 打印节点的输出情况来验证复杂结构的正确性
        for i in 0..6 {
            if let Some(output_node) = rack.operators.get(&1) {
                if let Some(port) = output_node.get_input_port(&i) {
                    println!("OperatorOutput input port {}: {}", i, port.borrow().value);
                }
            }
        }
        
    }
    
    fn loop_start(&mut self) {
        self.loop_start_time = Instant::now();
        self.display.frame_start();
    }
    
    fn loop_main(&mut self) {
        // let mut flag= 0;
        loop {
            // start loop
            self.loop_start();
            self.key_manager.get_keys();

            self.core.core_loop();

            // let path = env::current_exe().unwrap();
            // let folder_path = path.parent().unwrap();
            // let mut resource_path = PathBuf::from(folder_path);
            // resource_path.push("dip.png");
            // 
            // // main loop
            // self.display.draw_line(10,10,400,300,(255,0,0));
            // self.display.draw_rectangle(30, 40, 200, 100, (255,0,0),true);
            // self.display.draw_rectangle(50, 60, 180, 90, (0,150,0),false);
            // self.display.set_pixel_color(100, 100, (0,0,255));
            // self.display.draw_circle(200,200,30,(0,0,255,),true);
            // self.display.draw_rectangle_rounded(300,20,400,400,40,(255,255,255),true);
            // self.display.text("Hello World !",1,0,0, 2,2,(255,0,0));
            // self.display.text("@ Test ",2,10,100,2,10,(255,255,255));
            // self.display.image(resource_path, 0, 0);
            // flag += 1;
            // if flag == 400 {
            //     flag = 0;
            // }
            
            // end loop
            self.loop_end();
        }
    }
    
    fn loop_end(&mut self) {
        self.display.frame_update();
        if let Some(remaining) = self.fps.checked_sub(self.loop_start_time.elapsed()) {
            println!("{:?}", remaining);
            thread::sleep(remaining);
        }
    }
}

fn main() {
    let mut loop_manager = LoopManager::new(2.0);
    loop_manager.init();
    loop_manager.loop_main();
    // loop_manager.run_once_debug();
}
