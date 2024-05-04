use std::thread;
use std::time::{Duration, Instant};
use crate::view::display::display::Display;
use crate::view::interaction::key_manager::KeyManager;
use device_query::Keycode;

pub struct ViewContainer {
    //display
    loop_start_time: Instant,
    fps: Duration,
    display:Display,
    //pages
    pub page_0: Page0DataLoader,
    page_index: usize,
    //keyboard interaction
    key_manager: KeyManager
}

impl ViewContainer {
    pub fn new(fps:f32, track_number:usize) -> Self {
        ViewContainer{
            loop_start_time: Instant::now(),
            fps: Duration::from_secs_f32(1.0 / fps),
            display: Display::new(480, 480, 480 * 4,4),
            page_0: Page0DataLoader::new(track_number),
            page_index:0,
            key_manager: KeyManager::new(),
        }
    }
    
    pub fn frame_init (&mut self) {
        match self.page_index {
            0 => {
                for (i, d) in self.page_0.data_loader_blocks.iter().enumerate(){
                    let block_name = self.page_0.data_loader_blocks[i].get_block_name();
                    self.display.text(&block_name, 1, 10, 10+20*i, 1, 1, (255, 255, 255))
                }
            },
            _ => {todo!()},
        }
    }
    
    pub fn frame_start (&mut self) {
        self.loop_start_time = Instant::now();
        self.display.frame_start();
    }
    
    pub fn frame_main (&mut self) {
        self.key_manager.get_keys();
        
        match self.page_index {
            0 => {
                if !self.key_manager.keys.is_empty() {
                    match self.key_manager.keys[0] {
                        Keycode::Up => {
                            println!("up");
                            if self.page_0.focus_rect[1] !=0 {
                                self.page_0.focus_rect[1] -= 1;
                            }
                        },
                        Keycode::Down => {
                            println!("down");
                            if self.page_0.focus_rect[1] != self.page_0.data_loader_blocks.len() - 1 {
                                self.page_0.focus_rect[1] += 1;
                            }
                        },
                        Keycode::Left => {
                            println!("left");
                            if self.page_0.focus_rect[0] !=0 {
                                self.page_0.focus_rect[0] -= 1;
                            }
                        },
                        Keycode::Right => {
                            println!("right");
                            if self.page_0.focus_rect[0] != 1 {
                                self.page_0.focus_rect[0] += 1;
                            }
                        },
                        _ => println!("other"),
                    }
                }
                
                println!("{:?}", self.page_0.focus_rect);
                
                for (i, d) in self.page_0.data_loader_blocks.iter().enumerate(){
                    let block_name = self.page_0.data_loader_blocks[i].get_block_name();
                    self.display.text(&block_name, 1, 10, 10+20*i, 1, 1, (255, 0, 0))
                }
            },
            _ => {todo!()},
        }
    }
    
    pub fn frame_end (&mut self) {
        self.display.frame_update();
        if let Some(remaining) = self.fps.checked_sub(self.loop_start_time.elapsed()) {
            println!("{:?}", remaining);
            thread::sleep(remaining);
        } else {
            println!("Rendering Over Time")
        }
    }
}

pub struct Page0DataLoader {
    track_number:usize,
    //data loader rack
    data_loader_blocks: Vec<Box<dyn DataLoaderUiBlockInterface>>,
    //wave preview
    //ui focus
    focus_rect:[usize; 2],
}

impl Page0DataLoader {
    fn new(track_number:usize) -> Self {
        Page0DataLoader {
            track_number,
            data_loader_blocks: (0..track_number).map(|_| Box::new(EmptyLoaderUiBlock::new()) as Box<dyn DataLoaderUiBlockInterface>).collect(),
            focus_rect:[0,0],
        }
    }
    
    pub fn select_data_loader(&mut self, index:usize, data_loader:Box<dyn DataLoaderUiBlockInterface>) {
        self.data_loader_blocks[index] = data_loader;
    }
}

pub trait DataLoaderUiBlockInterface {
    fn update(&mut self);
    fn call_menu(&mut self);
    fn get_block_name(&self) -> String;
}

struct EmptyLoaderUiBlock {
    data_loader_name: String,
}

impl EmptyLoaderUiBlock {
    fn new() -> Self {
        EmptyLoaderUiBlock {
            data_loader_name: String::from("Empty"),
        }
    }
}

impl DataLoaderUiBlockInterface for EmptyLoaderUiBlock {
    fn update(&mut self) {
        todo!()
    }

    fn call_menu(&mut self) {
        todo!()
    }

    fn get_block_name(&self) -> String {
        self.data_loader_name.clone()
    }
}
pub struct FileLoaderUiBlock {
    data_loader_name: String,
}

impl FileLoaderUiBlock {
    pub fn new () -> Self {
        FileLoaderUiBlock{
            data_loader_name: String::from("FileLoader"),
        }
    }
}

impl DataLoaderUiBlockInterface for FileLoaderUiBlock {
    fn update(&mut self) {
        todo!()
    }

    fn call_menu(&mut self) {
        todo!()
    }

    fn get_block_name(&self) -> String {
        self.data_loader_name.clone()
    }
}


struct WavePreviewUiBlock {
    
}