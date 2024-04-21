use std::time::{Duration, Instant};
use std::thread;
use crate::user_interface::display::font::{font_pixel_operator_16, font_dot_digital_20};
use image::GenericImageView;
use image::io::Reader as ImageReader;

#[cfg(windows)]
use minifb::{Window, WindowOptions};

#[cfg(all(target_os = "linux", target_arch = "arm"))]
use framebuffer::Framebuffer;

pub struct Display {
    width: usize,
    height: usize,
    line_byte_length: usize,
    bytes_per_pixel: usize,
    buffer: Vec<u8>,
    frame_start_time: Instant,
    frame_time: Duration,
    #[cfg(windows)]
    window_win:Window,
    #[cfg(windows)]
    simulator_buffer: Vec<u32>,

    #[cfg(all(target_os = "linux", target_arch = "arm"))]
    window_linux:Framebuffer,
}

impl Display {
    pub fn new(width:usize, height:usize, line_byte_length:usize, bytes_per_pixel:usize, fps:f32) -> Self {
        let buffer= vec![0u8; line_byte_length * height];
        let simulator_buffer = vec![0u32; width * height];
        let frame_start_time = Instant::now();
        let frame_time = Duration::from_secs_f32(1.0 / fps);

        Display {
            width,
            height,
            line_byte_length,
            bytes_per_pixel,
            buffer,
            frame_start_time,
            frame_time,

            #[cfg(windows)]
            window_win: Window::new(
                "Display Simulator",
                width,
                height,
                WindowOptions::default(),
            ).expect("Unable to create window"),
            #[cfg(windows)]
            simulator_buffer,

            #[cfg(all(target_os = "linux", target_arch = "arm"))]
            window_linux:Framebuffer::new("/dev/fb0").expect("Unable to open framebuffer"),
        }
    }

    ///
    /// Frame starting
    ///
    pub fn frame_start (&mut self) {
        self.frame_start_time = Instant::now();
        self.buffer.fill(0);
    }

    ///
    /// frame processing
    ///
    pub fn set_pixel_color (&mut self, x:usize, y:usize, color:(u8,u8,u8)) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) * self.bytes_per_pixel;
            self.buffer[index + 2] = color.0;     // Red
            self.buffer[index + 1] = color.1; // Green
            self.buffer[index] = color.2; // Blue
        }
    }

    #[cfg(windows)]
    fn convert_buffer_to_simulator_all(&mut self) {
        for (i, chunk) in self.buffer.chunks(4).enumerate() {
            let r = u32::from(chunk[0]);
            let g = u32::from(chunk[1]) << 8;
            let b = u32::from(chunk[2]) << 16;
            self.simulator_buffer[i] = r | g | b;
        }
    }

    #[cfg(windows)]
    fn _convert_buffer_to_simulator_pixel(&mut self, x: usize, y: usize) {
        let start_index = y * self.line_byte_length + x * self.bytes_per_pixel;
        if start_index + 3 < self.buffer.len() {
            let r = u32::from(self.buffer[start_index]);
            let g = u32::from(self.buffer[start_index + 1]) << 8;
            let b = u32::from(self.buffer[start_index + 2]) << 16;
            self.simulator_buffer[y * self.width + x] = r | g | b;
        }
    }

    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: (u8, u8, u8)) {
        let dx = (x1 as isize - x0 as isize).abs();
        let dy = -(y1 as isize - y0 as isize).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy; // error value e_xy
        let mut x = x0 as isize;
        let mut y = y0 as isize;

        loop {
            // Set pixel color, skip Alpha channel
            let index = (y as usize * self.width + x as usize) * self.bytes_per_pixel;
            self.buffer[index + 2] = color.0;     // Red
            self.buffer[index + 1] = color.1; // Green
            self.buffer[index] = color.2; // Blue

            if x == x1 as isize && y == y1 as isize { break; }
            let e2 = 2 * err;
            if e2 >= dy { // e_xy+e_x > 0
                err += dy;
                x += sx;
            }
            if e2 <= dx { // e_xy+e_y < 0
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_rectangle(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: (u8, u8, u8), fill:bool) {
        // ensure x0, y0 is at left of x1, y1
        let (x0, x1) = if x0 < x1 { (x0, x1) } else { (x1, x0) };
        let (y0, y1) = if y0 < y1 { (y0, y1) } else { (y1, y0) };

        if fill {
            for y in y0..=y1 {
                for x in x0..=x1 {
                    let index = (y * self.width + x) * self.bytes_per_pixel;
                    self.buffer[index + 2] = color.0;     // Red
                    self.buffer[index + 1] = color.1; // Green
                    self.buffer[index] = color.2; // Blue
                }
            }
        } else {
            // draw top and bottom
            for x in x0..=x1 {
                let top_index = (y0 * self.width + x) * self.bytes_per_pixel;
                let bottom_index = (y1 * self.width + x) * self.bytes_per_pixel;

                self.buffer[top_index + 2] = color.0;     // Red
                self.buffer[top_index + 1] = color.1; // Green
                self.buffer[top_index] = color.2; // Blue

                self.buffer[bottom_index + 2] = color.0;     // Red
                self.buffer[bottom_index + 1] = color.1; // Green
                self.buffer[bottom_index ] = color.2; // Blue
            }

            // draw left and right, and skip conner
            for y in (y0 + 1)..y1 {
                let left_index = (y * self.width + x0) * self.bytes_per_pixel;
                let right_index = (y * self.width + x1) * self.bytes_per_pixel;
                self.buffer[left_index + 2] = color.0;     // Red
                self.buffer[left_index + 1] = color.1; // Green
                self.buffer[left_index] = color.2; // Blue

                self.buffer[right_index + 2] = color.0;     // Red
                self.buffer[right_index + 1] = color.1; // Green
                self.buffer[right_index] = color.2; // Blue
            }
        }
    }

    pub fn draw_rectangle_rounded(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, radius: usize, color: (u8, u8, u8), fill: bool) {
        // 保证坐标正确性
        let (x0, x1) = if x0 < x1 { (x0, x1) } else { (x1, x0) };
        let (y0, y1) = if y0 < y1 { (y0, y1) } else { (y1, y0) };

        // 绘制中间矩形部分
        if fill {
            // 填充时包含圆角内侧
            self.draw_rectangle(x0, y0 + radius, x1, y1 - radius, color, true);
            self.draw_rectangle(x0 + radius, y0, x1 - radius, y1, color, true);
        } else {
            // 仅绘制边框
            self.draw_rectangle(x0 + radius, y0, x1 - radius, y0, color, true); // 上边框
            self.draw_rectangle(x0 + radius, y1, x1 - radius, y1, color, true); // 下边框
            self.draw_rectangle(x0, y0 + radius, x0, y1 - radius, color, true); // 左边框
            self.draw_rectangle(x1, y0 + radius, x1, y1 - radius, color, true); // 右边框
        }

        // 绘制四个圆角
        // let corner_color = if fill { color } else { (0, 0, 0) }; // 如果不填充，使用透明色作为圆角颜色（这里假设(0,0,0)为透明色，实际应根据实际情况调整）
        self.draw_circle_quarter(x0 + radius, y0 + radius, radius, color, fill, 2); // 左上角
        self.draw_circle_quarter(x1 - radius, y0 + radius, radius, color, fill, 1); // 右上角
        self.draw_circle_quarter(x1 - radius, y1 - radius, radius, color, fill, 4); // 右下角
        self.draw_circle_quarter(x0 + radius, y1 - radius, radius, color, fill, 3); // 左下角
    }

    pub fn draw_circle_quarter(&mut self, cx: usize, cy: usize, radius: usize, color: (u8, u8, u8), fill: bool, quarter: u8) {
        let mut x = radius as isize;
        let mut y = 0isize;
        let mut err = 1 - x;

        while x >= y {
            if fill {
                match quarter {
                    1 => { // 右上象限
                        for i in cx as isize..=cx as isize + x {
                            self.set_pixel_color(i as usize, (cy as isize - y) as usize, color);
                        }
                        for i in cx as isize..=cx as isize + y {
                            self.set_pixel_color(i as usize, (cy as isize - x) as usize, color);
                        }
                    },
                    2 => { // 左上象限
                        for i in (cx as isize - x)..=cx as isize {
                            self.set_pixel_color(i as usize, (cy as isize - y) as usize, color);
                        }
                        for i in (cx as isize - y)..=cx as isize {
                            self.set_pixel_color(i as usize, (cy as isize - x) as usize, color);
                        }
                    },
                    3 => { // 左下象限
                        for i in (cx as isize - x)..=cx as isize {
                            self.set_pixel_color(i as usize, (cy as isize + y) as usize, color);
                        }
                        for i in (cx as isize - y)..=cx as isize {
                            self.set_pixel_color(i as usize, (cy as isize + x) as usize, color);
                        }
                    },
                    4 => { // 右下象限
                        for i in cx as isize..=cx as isize + x {
                            self.set_pixel_color(i as usize, (cy as isize + y) as usize, color);
                        }
                        for i in cx as isize..=cx as isize + y {
                            self.set_pixel_color(i as usize, (cy as isize + x) as usize, color);
                        }
                    },
                    _ => {}
                }
            } else {
                let points = match quarter {
                    1 => vec![(cx + x as usize, cy - y as usize), (cx + y as usize, cy - x as usize)],
                    2 => vec![(cx - x as usize, cy - y as usize), (cx - y as usize, cy - x as usize)],
                    3 => vec![(cx - x as usize, cy + y as usize), (cx - y as usize, cy + x as usize)],
                    4 => vec![(cx + x as usize, cy + y as usize), (cx + y as usize, cy + x as usize)],
                    _ => vec![],
                };

                for &(px, py) in &points {
                    self.set_pixel_color(px, py, color);
                }
            }

            y += 1;
            if err < 0 {
                err += 2 * y + 1;
            } else {
                x -= 1;
                err += 2 * (y - x + 1);
            }
        }
    }



    pub fn draw_circle(&mut self, cx: usize, cy: usize, radius: usize, color: (u8, u8, u8), fill: bool) {
        if fill {
            for y in (cy as isize - radius as isize)..=(cy as isize + radius as isize) {
                for x in (cx as isize - radius as isize)..=(cx as isize + radius as isize) {
                    let dx = x - cx as isize;
                    let dy = y - cy as isize;
                    if dx*dx + dy*dy <= (radius as isize)*(radius as isize) {
                        let index = (y as usize * self.width + x as usize) * self.bytes_per_pixel;
                        self.buffer[index + 2] = color.0;     // Red
                        self.buffer[index + 1] = color.1; // Green
                        self.buffer[index] = color.2; // Blue
                    }
                }
            }
        } else {
            let mut x = radius as isize;
            let mut y = 0isize;
            let mut err = 0isize;

            while x >= y {
                let points = [
                    (cx as isize + x, cy as isize + y),
                    (cx as isize + y, cy as isize + x),
                    (cx as isize - y, cy as isize + x),
                    (cx as isize - x, cy as isize + y),
                    (cx as isize - x, cy as isize - y),
                    (cx as isize - y, cy as isize - x),
                    (cx as isize + y, cy as isize - x),
                    (cx as isize + x, cy as isize - y),
                ];

                for &(px, py) in &points {
                    if px >= 0 && px < self.width as isize && py >= 0 && py < self.height as isize {
                        let index = (py as usize * self.width + px as usize) * self.bytes_per_pixel;
                        self.buffer[index + 2] = color.0;     // Red
                        self.buffer[index + 1] = color.1; // Green
                        self.buffer[index] = color.2; // Blue
                    }
                }

                y += 1;
                err += 1 + 2*y;
                if 2*(err-x) + 1 > 0 {
                    x -= 1;
                    err += 1 - 2*x;
                }
            }
        }
    }

    pub fn text(&mut self, text: &str, font:usize, x: usize, y: usize, scale: usize, spacing: usize, color: (u8, u8, u8)) {
        let mut x = x;
        let (glyphs_table, font_height) = match font {
            1 => (font_pixel_operator_16::FONT_LOOKUP_TABLE, font_pixel_operator_16::FONT_HEIGHT),
            2 => (font_dot_digital_20::FONT_LOOKUP_TABLE, font_dot_digital_20::FONT_HEIGHT),
            _ => (font_pixel_operator_16::FONT_LOOKUP_TABLE, font_pixel_operator_16::FONT_HEIGHT),
        };
        for c in text.chars() {
            let char_pixels = glyphs_table[c as usize - 32];
            let char_width = char_pixels.len() / font_height;
            if scale == 1 {
                // 如果scale为1，直接渲染，不进行放大
                for row in 0..font_height {
                    for col in 0..char_width {
                        let pixel_index = row * char_width + col; // 在char_pixels中的索引
                        if char_pixels[pixel_index] == 1 {
                            let offset = ((y + row) * self.width + x + col) * self.bytes_per_pixel;
                            self.buffer[offset + 2] = color.0; // R
                            self.buffer[offset + 1] = color.1; // G
                            self.buffer[offset] = color.2; // B
                        }
                    }
                }
            } else {
                // 如果scale不为1，进行放大
                for row in 0..font_height {
                    for col in 0..char_width {
                        let pixel_index = row * char_width + col;
                        if char_pixels[pixel_index] == 1 {
                            for dy in 0..scale {
                                for dx in 0..scale {
                                    let offset = ((y + row * scale + dy) * self.width + x + col * scale + dx) * self.bytes_per_pixel;
                                    self.buffer[offset + 2] = color.0; // R
                                    self.buffer[offset + 1] = color.1; // G
                                    self.buffer[offset] = color.2; // B
                                }
                            }
                        }
                    }
                }
            }
            // 更新下一个字符的起始位置，考虑scale和字符间的间距
            x += scale * char_width + spacing;
        }
    }

    pub fn image(&mut self, img_path:&str, start_x:usize, start_y:usize) {
        let img = ImageReader::open(img_path).unwrap().decode().unwrap();
        let (img_width, img_height) = img.dimensions();

        for y in 0.. img_height {
            for x in 0.. img_width {
                let dst_x = start_x + x as usize;
                let dst_y = start_y + y as usize;

                if dst_x >= self.width || dst_y >= self.height {
                    continue;
                }

                let pixel = img.get_pixel(x, y).0;
                if pixel[3] == 0 {
                    continue;
                }

                let idx = (dst_y * self.width + dst_x) * 4;
                if idx + 3 < self.buffer.len() {
                    self.buffer[idx] = pixel[2];    // B
                    self.buffer[idx + 1] = pixel[1];  // G
                    self.buffer[idx + 2] = pixel[0];  // R
                }
            }
        }
    }



    ///
    /// Frame end for updating
    ///
    pub fn frame_update (&mut self) {

        #[cfg(windows)]
        {
            // converting simulator buffer to framebuffer
            self.convert_buffer_to_simulator_all();

            // updating frame with buffer
            self.window_win.update_with_buffer(&self.simulator_buffer,self.width,self.height).unwrap();

            if !self.window_win.is_open(){
                std::process::exit(0);
            }
        }

        #[cfg(all(target_os = "linux", target_arch = "arm"))]
        self.window_linux.write_frame(&self.buffer);

        // keeping fps
        if let Some(remaining) = self.frame_time.checked_sub(self.frame_start_time.elapsed()) {
            thread::sleep(remaining);
        }
    }
}