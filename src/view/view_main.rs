use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};
use crate::view::display::display::Display;
use crate::view::interaction::key_manager::KeyManager;
use device_query::Keycode;

pub struct ViewContainer {
    //display
    loop_start_time: Instant,
    fps: Duration,
    display:Rc<RefCell<Display>>,
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
            display: Rc::new(RefCell::new(Display::new(480, 480, 480 * 4,4))),
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
                    self.display.borrow_mut().text(&block_name, 1, 10, 10+20*i, 1, 1, (255, 255, 255))
                }
                
                for (i, d) in self.page_0.wave_preview_blocks.iter().enumerate(){
                    let block_name = self.page_0.wave_preview_blocks[i].get_block_name();
                    self.display.borrow_mut().text(&block_name, 1, 100, 10+20*i, 1, 1, (255, 255, 255))
                }
            },
            _ => {todo!()},
        }
    }
    
    pub fn frame_start (&mut self) {
        self.loop_start_time = Instant::now();
        self.display.borrow_mut().frame_start();
    }
    
    pub fn frame_main (&mut self) {
        let selected_block_index = self.page_0.focus_rect;
        
        match self.page_index {
            0 => {
                self.process_key_input();
                println!("{:?}", self.page_0.focus_rect);
                
                let new_selected_block_index = self.page_0.focus_rect;
                if new_selected_block_index != selected_block_index {
                    if new_selected_block_index[0] != selected_block_index[0] {
                        self.frame_init();
                    }
                    
                    if new_selected_block_index[0] == 0 {
                        self.display.borrow_mut().text(&self.page_0.data_loader_blocks[selected_block_index[1]].get_block_name(), 1, 10, 10+20*selected_block_index[1], 1, 1, (255, 255, 255));
                        self.display.borrow_mut().text(&self.page_0.data_loader_blocks[new_selected_block_index[1]].get_block_name(), 1, 10, 10+20*new_selected_block_index[1], 1, 1, (255, 0, 0));
                    }
                    
                    if new_selected_block_index[0] == 1 {
                        self.display.borrow_mut().text(&self.page_0.wave_preview_blocks[selected_block_index[1]].get_block_name(), 1, 100, 10+20*selected_block_index[1], 1, 1, (255, 255, 255));
                        self.display.borrow_mut().text(&self.page_0.wave_preview_blocks[new_selected_block_index[1]].get_block_name(), 1, 100, 10+20*new_selected_block_index[1], 1, 1, (255, 0, 0));
                    }
                }
            },
            _ => {todo!()},
        }
    }
    
    pub fn frame_end (&mut self) {
        self.display.borrow_mut().frame_update();
        if let Some(remaining) = self.fps.checked_sub(self.loop_start_time.elapsed()) {
            println!("{:?}", remaining);
            thread::sleep(remaining);
        } else {
            println!("Rendering Over Time")
        }
    }

    fn process_key_input(&mut self) {
        self.key_manager.get_keys();
        if let Some(key) = self.key_manager.keys.first() {
            match key {
                Keycode::Up => self.navigate_vertical(-1),
                Keycode::Down => self.navigate_vertical(1),
                Keycode::Left => self.navigate_horizontal(-1),
                Keycode::Right => self.navigate_horizontal(1),
                _ => println!("Unhandled key press: {:?}", key),
            }
        }
    }

    fn navigate_vertical(&mut self, dir: isize) {
        let new_index = self.page_0.focus_rect[1] as isize + dir;
        if new_index >= 0 && new_index < self.page_0.data_loader_blocks.len() as isize {
            self.page_0.focus_rect[1] = new_index as usize;
            println!("Vertical navigation: {}", dir);
        }
    }

    fn navigate_horizontal(&mut self, dir: isize) {
        let new_index = self.page_0.focus_rect[0] as isize + dir;
        if new_index >= 0 && new_index < 2 {
            self.page_0.focus_rect[0] = new_index as usize;
            println!("Horizontal navigation: {}", dir);
        }
    }
}

pub struct Page0DataLoader {
    track_number:usize,
    //data loader rack
    data_loader_blocks: Vec<Box<dyn UiBlockInterface>>,
    //wave preview
    wave_preview_blocks: Vec<Box<dyn UiBlockInterface>>,
    //ui focus
    focus_rect:[usize; 2],
}

impl Page0DataLoader {
    fn new(track_number:usize) -> Self {
        Page0DataLoader {
            track_number,
            data_loader_blocks: (0..track_number).map(|_| Box::new(EmptyLoaderUiBlock::new()) as Box<dyn UiBlockInterface>).collect(),
            wave_preview_blocks: (0..track_number).map(|_| Box::new(WavePreviewUiBlock::new()) as Box<dyn UiBlockInterface>).collect(),
            focus_rect:[0,0],
        }
    }
    
    pub fn select_data_loader(&mut self, index:usize, data_loader:Box<dyn UiBlockInterface>) {
        self.data_loader_blocks[index] = data_loader;
    }
}

pub trait UiBlockInterface {
    fn call_menu(&mut self);
    fn get_block_name(&self) -> String;
}

struct EmptyLoaderUiBlock {
    data_loader_name: String,
    is_selected: bool,
}

impl EmptyLoaderUiBlock {
    fn new() -> Self {
        EmptyLoaderUiBlock {
            data_loader_name: String::from("Empty"),
            is_selected: false,
        }
    }
}

impl UiBlockInterface for EmptyLoaderUiBlock {
    fn call_menu(&mut self) {
        todo!()
    }

    fn get_block_name(&self) -> String {
        self.data_loader_name.clone()
    }
}
pub struct FileLoaderUiBlock {
    data_loader_name: String,
    is_selected: bool,
}

impl FileLoaderUiBlock {
    pub fn new () -> Self {
        FileLoaderUiBlock{
            data_loader_name: String::from("FileLoader"),
            is_selected: false,
        }
    }
}

impl UiBlockInterface for FileLoaderUiBlock {
    fn call_menu(&mut self) {
        todo!()
    }

    fn get_block_name(&self) -> String {
        self.data_loader_name.clone()
    }
}


struct WavePreviewUiBlock {
    wave_preview_name: String,
    is_selected: bool,
}

impl WavePreviewUiBlock {
    pub fn new () -> Self {
        WavePreviewUiBlock{
            wave_preview_name: String::from("WavePreview"),
            is_selected: false,
        }
    }
}

impl UiBlockInterface for WavePreviewUiBlock {
    fn call_menu(&mut self) {
        todo!()
    }

    fn get_block_name(&self) -> String {
        self.wave_preview_name.clone()
    }
}