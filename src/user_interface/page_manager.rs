trait PageBehavior {
    fn render(&self);
    fn handle_key_event(&mut self, key: u8);
}

struct PageManager {
    pages: Vec<Box<dyn PageBehavior>>,
    current_page_index: usize,
}

impl PageManager {
    fn new (pages: Vec<Box<dyn PageBehavior>>) -> Self {
        PageManager{
            pages,
            current_page_index: 0,
        }
    }
    
    fn switch_to_page(&mut self, index:usize){
        if index < self.pages.len() {
            self.current_page_index = index;
        }
    }

    fn render_current_page(&self) {
        if let Some(current_page) = self.pages.get(self.current_page_index) {
            current_page.render();
        }
    }
    
    fn handle_key_event(&mut self, key:u8) {
        if let Some(current_page) = self.pages.get_mut(self.current_page_index) {
            current_page.handle_key_event(key);
        }
    }
}

struct PageDataLoader {
    
}

impl PageBehavior for PageDataLoader {
    fn render(&self) {
        todo!()
    }

    fn handle_key_event(&mut self, key: u8) {
        todo!()
    }
}

struct  PageWaveContainer {
    
}

impl PageBehavior for PageWaveContainer {
    fn render(&self) {
        todo!()
    }

    fn handle_key_event(&mut self, key: u8) {
        todo!()
    }
}

struct PageOperatorRack {
    
}

impl PageBehavior for PageOperatorRack{
    fn render(&self) {
        todo!()
    }

    fn handle_key_event(&mut self, key: u8) {
        todo!()
    }
}

struct PageDataOutput {
    
}

impl PageBehavior for PageDataOutput {
    fn render(&self) {
        todo!()
    }

    fn handle_key_event(&mut self, key: u8) {
        todo!()
    }
}




