use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};
use crate::view::display::display::Display;
use crate::view::interaction::key_manager::KeyManager;
use log::{debug,info};


/// # Main View Container
pub struct ViewContainer {
    loop_start_time: Instant,
    fps: Duration,
    display:Rc<RefCell<Display>>,
    page_0: Page0DataLoader,
    page_1: Page1WaveEditor,
    page_index: Rc<RefCell<usize>>,
    page_index_prvious: usize,
    key_manager: Rc<RefCell<KeyManager>>,
}

impl ViewContainer {
    pub fn new(fps:f32, track_number:usize) -> Self {
        let display = Rc::new(RefCell::new(Display::new(480, 480, 480 * 4,4)));
        let display_ref = display.clone();
        let key_manager = Rc::new(RefCell::new(KeyManager::new()));
        let page_index = Rc::new(RefCell::new(0));
        let page_0 = Page0DataLoader::new(track_number, display_ref.clone(), key_manager.clone(), page_index.clone());
        let page_1 = Page1WaveEditor::new(track_number, display_ref.clone(), key_manager.clone(), page_index.clone());

        ViewContainer{
            loop_start_time: Instant::now(),
            fps: Duration::from_secs_f32(1.0 / fps),
            display,
            page_0,
            page_1,
            key_manager,
            page_index,
            page_index_prvious: 0,
        }
    }

    pub fn frame_init (&mut self) {
        self.page_0.page_view_init();
        self.page_1.page_view_init();
    }

    pub fn frame_start (&mut self) {
        self.loop_start_time = Instant::now();
        self.display.borrow_mut().frame_start();
    }

    pub fn frame_main (&mut self) {

        let current_page_index = *self.page_index.borrow();

        if current_page_index != self.page_index_prvious {
            match current_page_index {
                0 => {
                    self.page_0.page_view_back();
                },
                1 => {
                    self.page_1.page_view_back();
                },
                _ => {todo!()},
            }
            self.page_index_prvious = current_page_index;
        }

        match current_page_index {
            0 => {
                self.page_0.page_view_update();
            },
            1 => {
                self.page_1.page_view_update();
            },
            _ => {todo!()},
        }
    }

    pub fn frame_end (&mut self) {
        self.display.borrow_mut().frame_update();
        if let Some(remaining) = self.fps.checked_sub(self.loop_start_time.elapsed()) {
            // info!("{:?}", remaining);
            thread::sleep(remaining);
        } else {
            // debug!("Rendering Over Time")
        }
    }
}

pub trait PageInterface {
    fn page_view_init(&mut self);
    fn page_view_back(&mut self);
    fn process_key_input(&mut self);
    fn process_key_input_block_menu(&mut self);
    fn navigate_vertical(&mut self, dir: isize);
    fn navigate_horizontal(&mut self, dir: isize);
    fn call_block_menu(&mut self);
    fn call_page(&mut self);
    fn update_page_index(&mut self, index: usize);
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
    key_manager: Rc<RefCell<KeyManager>>,
    block_menu_called: bool,
    page_index_ref: Rc<RefCell<usize>>,
}

impl Page0DataLoader {
    fn new(track_number: usize, display_ref: Rc<RefCell<Display>>, key_manager: Rc<RefCell<KeyManager>>, page_index_ref:Rc<RefCell<usize>>) -> Self {
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

        // debug!("{:?}", block_coordinates);

        Page0DataLoader {
            display_ref,
            track_number,
            data_loader_blocks: (0..track_number).map(|i| Box::new(EmptyLoaderUiBlock::new(display_ui_block_ref.clone(), block_coordinates[0][i])) as Box<dyn UiBlockInterface>).collect(),
            wave_preview_blocks: (0..track_number).map(|i| Box::new(WavePreviewUiBlock::new(display_ui_block_ref.clone(), block_coordinates[1][i])) as Box<dyn UiBlockInterface>).collect(),
            block_coordinates,
            focus_rect: [0, 0],
            key_manager,
            block_menu_called: false,
            page_index_ref,
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

    fn page_view_back(&mut self) {
        self.display_ref.borrow_mut().clean();
        for i in 0..self.track_number {
            self.data_loader_blocks[i].block_view_update();
            self.wave_preview_blocks[i].block_view_update();
        }
    }

    fn process_key_input(&mut self) {
        let key = self.key_manager.borrow_mut().check_keys();
        if let Some(first_key) = key.get(0) {
            debug!("{:?}", first_key);
            match first_key.as_str() {
                "Up" => self.navigate_vertical(-1),
                "Down" => self.navigate_vertical(1),
                "Left" => self.navigate_horizontal(-1),
                "Right" => self.navigate_horizontal(1),
                "2" => self.update_page_index(1),
                "M" => self.block_menu_called = true,
                _ => {},
            }
        }
    }

    fn process_key_input_block_menu(&mut self) {
        let key = self.key_manager.borrow_mut().check_keys();
        if let Some(first_key) = key.get(0) {
            match first_key.as_str() {
                "Up" => {
                    if self.focus_rect[0] == 0 {
                        self.data_loader_blocks[self.focus_rect[1]].block_key_input("Up");
                    }
                },
                "Down" => {
                    if self.focus_rect[0] == 0 {
                        self.data_loader_blocks[self.focus_rect[1]].block_key_input("Down");
                    }
                },
                "Left" =>  {
                    if self.focus_rect[0] == 0 {
                        self.data_loader_blocks[self.focus_rect[1]].block_key_input("Left");
                    }
                },
                "Right" => {
                    if self.focus_rect[0] == 0 {
                        self.data_loader_blocks[self.focus_rect[1]].block_key_input("Right");
                    }
                },
                "M" => self.call_page(),
                _ => {},
            }
        }
    }

    fn navigate_vertical(&mut self, dir: isize) {
        let new_index = self.focus_rect[1] as isize + dir;
        if new_index >= 0 && new_index < self.data_loader_blocks.len() as isize {
            self.focus_rect[1] = new_index as usize;
            // debug!("Vertical navigation: {}", dir);
        }
    }

    fn navigate_horizontal(&mut self, dir: isize) {
        let new_index = self.focus_rect[0] as isize + dir;
        if new_index >= 0 && new_index < 2 {
            self.focus_rect[0] = new_index as usize;
            // debug!("Horizontal navigation: {}", dir);
        }
    }
    
    fn call_block_menu(&mut self) {
        match self.focus_rect[0] {
            0 => self.data_loader_blocks[self.focus_rect[1]].call_menu(),
            1 => self.wave_preview_blocks[self.focus_rect[1]].call_menu(),
            _ => {},
        }
    }
    
    fn call_page(&mut self) {
        self.display_ref.borrow_mut().clean();
        for i in 0..self.track_number {
            self.data_loader_blocks[i].block_view_update();
            self.wave_preview_blocks[i].block_view_update();
        }
        self.block_menu_called = false
    }

    fn update_page_index(&mut self, index:usize){
        *self.page_index_ref.borrow_mut() = index;
    }

    fn page_view_update(&mut self) {

        if self.block_menu_called {
            self.call_block_menu();
            self.process_key_input_block_menu();

        } else {
            let selected_block_index = self.focus_rect;
            self.process_key_input();
            // debug!("{:?}", self.focus_rect);

            // block selection
            let new_selected_block_index = self.focus_rect;

            if new_selected_block_index != selected_block_index {
                self.data_loader_blocks[selected_block_index[1]].set_selected(false);
                self.wave_preview_blocks[selected_block_index[1]].set_selected(false);
                if new_selected_block_index[0] == 0 {
                    self.data_loader_blocks[new_selected_block_index[1]].set_selected(true);
                } else {
                    self.wave_preview_blocks[new_selected_block_index[1]].set_selected(true);
                }

                for i in 0..self.track_number {
                    self.data_loader_blocks[i].block_view_update();
                    self.wave_preview_blocks[i].block_view_update();
                }
            }
        }
    }
}

pub trait UiBlockInterface {
    fn block_view_update(&mut self);
    fn call_menu(&mut self);
    fn get_block_name(&self) -> String;
    fn set_selected(&mut self, is_selected: bool);
    fn block_key_input(&mut self, key: &str);
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
    menu: EmptyBlockMenu,
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
            menu: EmptyBlockMenu::new(),
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
        // debug!("{:?}", self.coordinate);
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
        // debug!("Menu Called: {}", self.data_loader_name);
        let mut display = self.display_ref.borrow_mut();
        display.draw_rectangle(50,50,400,400,(0,100,0),true);
        display.text("Empty Block Menu", 2, 70, 60, 2, 8, (255,255,255));
        for (index, item) in self.menu.items.iter().enumerate() {
            let color = if self.menu.selected_index == index {
                (255, 0, 0)
            } else {
                (255, 255, 255)
            };
            display.text(item, 1, 70, 100 + index * 20, 1, 1, color);
        }
    }

    fn get_block_name(&self) -> String {
        self.data_loader_name.clone()
    }

    fn set_selected(&mut self, is_selected: bool) {
        self.is_selected = is_selected;
    }

    fn block_key_input(&mut self, key: &str) {
        match key {
            "Up" => {
                if self.menu.selected_index > 0 {
                    self.menu.selected_index -= 1;
                }
            },
            "Down" => {
                if self.menu.selected_index < self.menu.items.len() - 1 {
                    self.menu.selected_index += 1;
                }
            },
            "Left" => debug!("Left"),
            "Right" => {
                self.menu.execute();
            },
            _ => {},
        }
    }
}

struct EmptyBlockMenu {
    items: Vec<String>,
    pub selected_index: usize,
}

impl EmptyBlockMenu {
    fn new() -> Self {
        EmptyBlockMenu {
            items: vec![String::from("Load Wave From File"), String::from("Create Wave Generator"), String::from("Create Sensor Reader")],
            selected_index: 0,
        }
    }

    fn execute(&self) {
        debug!("Menu Executed: {}", self.items[self.selected_index]);
    }
}


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
        // debug!("{:?}", self.coordinate);
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

    fn block_key_input(&mut self, key: &str) {
        todo!()
    }
}


pub trait UiWaveEditorInterface {
    fn block_view_update(&mut self);
    fn call_menu(&mut self);
    fn get_block_name(&self) -> String;
    fn set_selected(&mut self, is_selected: bool);
    fn block_key_input(&mut self, key: &str);
}

/// # Page 1
pub struct Page1WaveEditor {
    display_ref: Rc<RefCell<Display>>,
    track_number:usize,
    wave_edit_blocks: Vec<Box<dyn WaveEditorUiBlockInterface>>,
    wave_preview_block_coordinates: Vec<[usize;2]>,
    focus_rect:usize,
    key_manager: Rc<RefCell<KeyManager>>,
    block_menu_called: bool,
    page_index_ref: Rc<RefCell<usize>>,
}

impl Page1WaveEditor {
    fn new(track_number: usize, display_ref: Rc<RefCell<Display>>, key_manager: Rc<RefCell<KeyManager>>, page_index_ref:Rc<RefCell<usize>>) -> Self {
        let display_ui_block_ref = display_ref.clone();
        let gap_height = 480 / track_number;
        let mut wave_preview_block_coordinates = Vec::new();
        for i in 0..track_number {
            wave_preview_block_coordinates.push([10, gap_height * i]);
        }

        Page1WaveEditor{
            display_ref,
            track_number,
            wave_edit_blocks: (0..track_number).map(|i| Box::new(WaveEditorUiBlock::new(display_ui_block_ref.clone(), wave_preview_block_coordinates[i])) as Box<dyn WaveEditorUiBlockInterface>).collect(),
            wave_preview_block_coordinates,
            focus_rect: 0,
            key_manager,
            block_menu_called: false,
            page_index_ref,
        }
    }
}

impl PageInterface for Page1WaveEditor {
    fn page_view_init(&mut self) {
        // self.display_ref.borrow_mut().clean();
        // // display.text("Page 1 Wave Editor", 1, 10, 10, 2, 8, (255,255,255));
        // for i in 0..self.track_number {
        //     self.wave_edit_blocks[i].block_view_update();
        // }
        self.wave_edit_blocks[0].set_selected(true);
    }

    fn page_view_back(&mut self) {
        self.display_ref.borrow_mut().clean();
        // display.text("Page 1 Wave Editor", 1, 10, 10, 2, 8, (255,255,255));
        for i in 0..self.track_number {
            self.wave_edit_blocks[i].block_view_update();
        }
    }

    fn process_key_input(&mut self) {
        let key = self.key_manager.borrow_mut().check_keys();
        if let Some(first_key) = key.get(0) {
            debug!("{:?}", first_key);
            match first_key.as_str() {
                "1" => self.update_page_index(0),
                _ => {},
            }
        }
    }

    fn process_key_input_block_menu(&mut self) {
        todo!()
    }

    fn navigate_vertical(&mut self, dir: isize) {
        todo!()
    }

    fn navigate_horizontal(&mut self, dir: isize) {
        todo!()
    }

    fn call_block_menu(&mut self) {
        todo!()
    }

    fn call_page(&mut self) {
        todo!()
    }

    fn update_page_index(&mut self, index:usize){
        *self.page_index_ref.borrow_mut() = index;
        debug!("Page Index Updated: {}", index);
    }

    fn page_view_update(&mut self) {
        self.process_key_input();
    }
}


trait WaveEditorUiBlockInterface {
    fn block_view_update(&mut self);
    fn set_selected(&mut self, is_selected: bool);
}

struct WaveEditorUiBlock {
    display_ref: Rc<RefCell<Display>>,
    wave_editor_block_name: String,
    is_selected: bool,
    coordinate: [usize; 2],
    coordinate_shift_x: usize,
    coordinate_shift_y: usize,
    block_ui_width: usize,
    block_ui_height: usize,
}

impl WaveEditorUiBlock {
    pub fn new (display_ref:Rc<RefCell<Display>>, coordinate:[usize;2]) -> Self {
        WaveEditorUiBlock{
            display_ref,
            wave_editor_block_name: String::from("WaveEditor"),
            is_selected: false,
            coordinate,
            coordinate_shift_x: 10,
            coordinate_shift_y: 10,
            block_ui_width: 150,
            block_ui_height: 50,
        }
    }
}

impl WaveEditorUiBlockInterface for WaveEditorUiBlock {
    fn block_view_update(&mut self) {
        let mut color = (30,30,30);
        if self.is_selected {
            color = (100,30,30);
        }

        let mut display = self.display_ref.borrow_mut();

        display.draw_rectangle(self.coordinate[0]+self.coordinate_shift_x,
                               self.coordinate[1]+self.coordinate_shift_y,
                               self.coordinate[0]+self.block_ui_width+self.coordinate_shift_x,
                               self.coordinate[1]+self.block_ui_height+self.coordinate_shift_y,
                               color,
                               true);
        display.text(&self.wave_editor_block_name, 1,
                     self.coordinate[0]+self.coordinate_shift_x+5,
                     self.coordinate[1]+self.coordinate_shift_y+5,
                     1, 1, (0, 255, 0));

    }

    fn set_selected(&mut self, is_selected:bool) {
        self.is_selected = is_selected;
    }
}
