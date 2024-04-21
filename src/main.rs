mod user_interface;
mod core;

use user_interface::display::display::*;
use rand::Rng;

fn main() {
    let mut display = Display::new(480, 480, 480 * 4,4,30.0);
    // let mut rng = rand::thread_rng();
    let mut flag= 0;
    loop{
        display.frame_start();

        display.draw_line(10,10,400,300,(255,0,0));
        display.draw_rectangle(30, 40, 200, 100, (255,0,0),true);
        display.draw_rectangle(50, 60, 180, 90, (0,150,0),false);
        display.set_pixel_color(100, 100, (0,0,255));
        display.draw_circle(200,200,30,(0,0,255,),true);
        display.draw_rectangle_rounded(300,20,400,400,40,(255,255,255),true);
        display.text("Hello World !",1,0,0, 2,2,(255,0,0));
        display.text("@ Test ",2,10,100,2,10,(255,255,255));
        display.image("icon.png", flag, flag);
        flag += 1;
        if flag == 400 {
            flag = 0;
        }

        display.frame_update();
    }
}
