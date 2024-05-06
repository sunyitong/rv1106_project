use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};
use crate::view::display::display::Display;
use crate::view::interaction::key_manager::KeyManager;
use device_query::Keycode;


/// # Main View Container
pub struct ViewContainer {
    loop_start_time: Instant,
    fps: Duration,
    display:Rc<RefCell<Display>>,
    pages: Vec<Box<dyn PageInterface>>,
    page_index: usize,
    key_manager: Rc<RefCell<KeyManager>>,
}

impl ViewContainer {
    pub fn new(fps:f32, track_number:usize) -> Self {
        let display = Rc::new(RefCell::new(Display::new(480, 480, 480 * 4,4)));
        let display_ref = display.clone();
        let key_manager = Rc::new(RefCell::new(KeyManager::new(10.0)));
        let page_0 = Box::new(Page0DataLoader::new(track_number, display_ref.clone(), key_manager.clone())) as Box<dyn PageInterface>;

        ViewContainer{
            loop_start_time: Instant::now(),
            fps: Duration::from_secs_f32(1.0 / fps),
            display,
            pages: vec![page_0],
            key_manager,
            page_index: 0,
        }
    }

    pub fn frame_init (&mut self) {
        for i in self.pages.iter_mut() {
            i.page_view_init();
        }
    }

    pub fn frame_start (&mut self) {
        self.loop_start_time = Instant::now();
        self.display.borrow_mut().frame_start();
    }

    pub fn frame_main (&mut self) {

        match self.page_index {
            0 => {
                let page = self.pages.get_mut(self.page_index).unwrap();
                page.page_view_update();
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
}

pub trait PageInterface {
    fn page_view_init(&mut self);
    fn process_key_input(&mut self);
    fn navigate_vertical(&mut self, dir: isize);
    fn navigate_horizontal(&mut self, dir: isize);
    fn page_view_update(&mut self);
}

/// # Page 0
pub struct Page0DataLoader {
    display_ref: Rc<RefCell<Display>>,
    track_number:usize,
    data_loader_blocks: Vec<Box<dyn UiBlockInterface>>,
    wave_preview_blocks: Vec<Box<dyn UiBlockInterface>>,
    block_coordinates: Vec<Vec<[usize;2]>>,
    focus_rect:[usize; 2],
    key_manager: Rc<RefCell<KeyManager>>
}

impl Page0DataLoader {
    fn new(track_number: usize, display_ref: Rc<RefCell<Display>>, key_manager: Rc<RefCell<KeyManager>>) -> Self {
        let display_ui_block_ref = display_ref.clone();
        let mut block_coordinates = Vec::new();
        let mut data_loader_block_coordinates = Vec::new();
        let gap_height = 480 / track_number;
        for i in 0..track_number {
            data_loader_block_coordinates.push([0, gap_height * i]);
        }
        let mut wave_preview_block_coordinates = Vec::new();
        for i in 0..track_number {
            wave_preview_block_coordinates.push([160, gap_height * i]);
        }
        block_coordinates.push(data_loader_block_coordinates);
        block_coordinates.push(wave_preview_block_coordinates);

        // println!("{:?}", block_coordinates);

        Page0DataLoader {
            display_ref,
            track_number,
            data_loader_blocks: (0..track_number).map(|i| Box::new(EmptyLoaderUiBlock::new(display_ui_block_ref.clone(), block_coordinates[0][i])) as Box<dyn UiBlockInterface>).collect(),
            wave_preview_blocks: (0..track_number).map(|i| Box::new(WavePreviewUiBlock::new(display_ui_block_ref.clone(), block_coordinates[1][i])) as Box<dyn UiBlockInterface>).collect(),
            block_coordinates,
            focus_rect: [0, 0],
            key_manager,
        }
    }

    pub fn select_data_loader(&mut self, index: usize, data_loader: Box<dyn UiBlockInterface>) {
        self.data_loader_blocks[index] = data_loader;
    }
}

impl PageInterface for Page0DataLoader {
    fn page_view_init(&mut self) {
        self.data_loader_blocks[0].set_selected(true);
        for i in 0..self.track_number {
            self.data_loader_blocks[i].block_view_update();
            self.wave_preview_blocks[i].block_view_update();
        }
    }

    fn process_key_input(&mut self) {
        let key = {
            let mut key_manager_ref = self.key_manager.borrow_mut();
            key_manager_ref.get_key()
        };
        println!("{:?}", key);
        match key.as_str() {
            "Up" => self.navigate_vertical(-1),
            "Down" => self.navigate_vertical(1),
            "Left" => self.navigate_horizontal(-1),
            "Right" => self.navigate_horizontal(1),
            _ => {},
        }
    }

    fn navigate_vertical(&mut self, dir: isize) {
        let new_index = self.focus_rect[1] as isize + dir;
        if new_index >= 0 && new_index < self.data_loader_blocks.len() as isize {
            self.focus_rect[1] = new_index as usize;
            println!("Vertical navigation: {}", dir);
        }
    }

    fn navigate_horizontal(&mut self, dir: isize) {
        let new_index = self.focus_rect[0] as isize + dir;
        if new_index >= 0 && new_index < 2 {
            self.focus_rect[0] = new_index as usize;
            println!("Horizontal navigation: {}", dir);
        }
    }

    fn page_view_update(&mut self) {
        let selected_block_index = self.focus_rect;
        self.process_key_input();
        println!("{:?}", self.focus_rect);

        let new_selected_block_index = self.focus_rect;

        if new_selected_block_index != selected_block_index {
            self.data_loader_blocks[selected_block_index[1]].set_selected(false);
            self.wave_preview_blocks[selected_block_index[1]].set_selected(false);
            if new_selected_block_index[0] == 0 {
                self.data_loader_blocks[new_selected_block_index[1]].set_selected(true);
            } else {
                self.wave_preview_blocks[new_selected_block_index[1]].set_selected(true);
                println!("Wave Preview Selected");
            }

            for i in 0..self.track_number {
                self.data_loader_blocks[i].block_view_update();
                self.wave_preview_blocks[i].block_view_update();
            }
        }
    }
}

pub trait UiBlockInterface {
    fn block_view_update(&mut self);
    fn call_menu(&mut self);
    fn get_block_name(&self) -> String;
    fn set_selected(&mut self, is_selected: bool);
}

/// # UI Block: Empty_Data_Loader
struct EmptyLoaderUiBlock {
    display_ref: Rc<RefCell<Display>>,
    data_loader_name: String,
    is_selected: bool,
    coordinate: [usize; 2],
    coordinate_shift_x: usize,
    coordinate_shift_y: usize,
    block_ui_width: usize,
    block_ui_height: usize,
}

impl EmptyLoaderUiBlock {
    fn new(display_ref: Rc<RefCell<Display>>, coordinate:[usize; 2]) -> Self {
        EmptyLoaderUiBlock {
            display_ref,
            data_loader_name: String::from("Empty"),
            is_selected: false,
            coordinate,
            coordinate_shift_x: 10,
            coordinate_shift_y: 10,
            block_ui_width: 80,
            block_ui_height: 50,

        }
    }
}

impl UiBlockInterface for EmptyLoaderUiBlock {
    fn block_view_update(&mut self) {
        let mut color = (30,30,30);
        if self.is_selected {
            color = (100,30,30);
        }
        let mut display = self.display_ref.borrow_mut();
        println!("{:?}", self.coordinate);
        display.draw_rectangle(self.coordinate[0]+self.coordinate_shift_x,
                               self.coordinate[1]+self.coordinate_shift_y,
                               self.coordinate[0]+self.block_ui_width+self.coordinate_shift_x,
                               self.coordinate[1]+self.block_ui_height+self.coordinate_shift_y,
                               color,
                               true);
        display.text(&self.data_loader_name, 1,
                     self.coordinate[0]+self.coordinate_shift_x+5,
                     self.coordinate[1]+self.coordinate_shift_y+5,
                     1, 1, (0, 255, 0));
    }

    fn call_menu(&mut self) {
        todo!()
    }

    fn get_block_name(&self) -> String {
        self.data_loader_name.clone()
    }

    fn set_selected(&mut self, is_selected: bool) {
        self.is_selected = is_selected;
    }
}

// pub struct FileLoaderUiBlock {
//     display_ref: Rc<RefCell<Display>>,
//     data_loader_name: String,
//     is_selected: bool,
// }
//
// impl FileLoaderUiBlock {
//     pub fn new (display_ref:Rc<RefCell<Display>>) -> Self {
//         FileLoaderUiBlock{
//             display_ref,
//             data_loader_name: String::from("FileLoader"),
//             is_selected: false,
//         }
//     }
// }
//
// impl UiBlockInterface for FileLoaderUiBlock {
//     fn block_view_init(&mut self) {
//         todo!()
//     }
//
//     fn call_menu(&mut self) {
//         todo!()
//     }
//
//     fn get_block_name(&self) -> String {
//         self.data_loader_name.clone()
//     }
// }


/// # UI Block: Wave Preview
struct WavePreviewUiBlock {
    display_ref: Rc<RefCell<Display>>,
    wave_preview_name: String,
    is_selected: bool,
    coordinate: [usize; 2],
    coordinate_shift_x: usize,
    coordinate_shift_y: usize,
    block_ui_width: usize,
    block_ui_height: usize,
}

impl WavePreviewUiBlock {
    pub fn new (display_ref:Rc<RefCell<Display>>, coordinate:[usize;2]) -> Self {
        WavePreviewUiBlock{
            display_ref,
            wave_preview_name: String::from("WavePreview"),
            is_selected: false,
            coordinate,
            coordinate_shift_x: 10,
            coordinate_shift_y: 10,
            block_ui_width: 80,
            block_ui_height: 50,
        }
    }
}

impl UiBlockInterface for WavePreviewUiBlock {
    fn block_view_update(&mut self) {
        let mut color = (30,30,30);
        if self.is_selected {
            color = (100,30,30);
        }
        let mut display = self.display_ref.borrow_mut();
        println!("{:?}", self.coordinate);
        display.draw_rectangle(self.coordinate[0]+self.coordinate_shift_x,
                               self.coordinate[1]+self.coordinate_shift_y,
                               self.coordinate[0]+self.block_ui_width+self.coordinate_shift_x,
                               self.coordinate[1]+self.block_ui_height+self.coordinate_shift_y,
                               color,
                               true);
        display.text(&self.wave_preview_name, 1,
                     self.coordinate[0]+self.coordinate_shift_x+5,
                     self.coordinate[1]+self.coordinate_shift_y+5,
                     1, 1, (0, 255, 0));
    }

    fn call_menu(&mut self) {
        todo!()
    }

    fn get_block_name(&self) -> String {
        self.wave_preview_name.clone()
    }

    fn set_selected(&mut self, is_selected: bool) {
        self.is_selected = is_selected;
    }
}