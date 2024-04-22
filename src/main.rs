mod user_interface;
mod core;

use std::thread;
use std::time::{Duration, Instant};
use user_interface::display::display::*;
use rand::Rng;
use core::core::*;

struct LoopManager {
    loop_start_time: Instant,
    fps: Duration,
    display:Display,
    core: Core,
}

impl LoopManager {
    fn new (fps: f32) -> Self {
        LoopManager{
            loop_start_time: Instant::now(),
            fps: Duration::from_secs_f32(1.0 / fps),
            display: Display::new(480, 480, 480 * 4,4),
            core: Core::new(6),
        }
    }
    
    fn loop_start(&mut self) {
        self.loop_start_time = Instant::now();
        self.display.frame_start();
    }
    
    fn loop_main(&mut self) {
        let mut flag= 0;
        loop {
            // start loop
            self.loop_start();

            self.core.core_loop();

            // main loop
            self.display.draw_line(10,10,400,300,(255,0,0));
            self.display.draw_rectangle(30, 40, 200, 100, (255,0,0),true);
            self.display.draw_rectangle(50, 60, 180, 90, (0,150,0),false);
            self.display.set_pixel_color(100, 100, (0,0,255));
            self.display.draw_circle(200,200,30,(0,0,255,),true);
            self.display.draw_rectangle_rounded(300,20,400,400,40,(255,255,255),true);
            self.display.text("Hello World !",1,0,0, 2,2,(255,0,0));
            self.display.text("@ Test ",2,10,100,2,10,(255,255,255));
            self.display.image("icon.png", flag, flag);
            flag += 1;
            if flag == 400 {
                flag = 0;
            }
            
            // end loop
            self.loop_end()
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
    let mut loop_manager = LoopManager::new(30.0);
    loop_manager.loop_main();
}
