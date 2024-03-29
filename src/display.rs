extern crate sdl2;

use self::sdl2::render::Canvas;
use self::sdl2::video::Window;

use crate::core::{GraphicsMemory, DISPLAY_HEIGHT, DISPLAY_WIDTH};

/**
The video sub-system used to render things on a canvas via sdl2

SDL2 Reference: https://docs.rs/sdl2/0.32.2/sdl2/
*/

const SCALE: u32 = 16;
const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;

lazy_static! {
    static ref COLOR_RED: sdl2::pixels::Color = sdl2::pixels::Color::RGB(255, 0, 0);
    static ref COLOR_BLUE: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0, 0, 255);
    static ref COLOR_GREEN: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0, 255, 0);
    static ref COLOR_BLACK: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0, 0, 0);
    static ref COLOR_WHITE: sdl2::pixels::Color = sdl2::pixels::Color::RGB(255, 255, 255);
}

pub struct VideoDisplay {
    pub canvas: sdl2::render::Canvas<Window>,
}

impl VideoDisplay {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem: sdl2::VideoSubsystem = sdl_context.video().unwrap();
        let canvas: Canvas<Window> = VideoDisplay::get_canvas(&video_subsystem);
        VideoDisplay { canvas }
    }

    pub fn init_window(video: &sdl2::VideoSubsystem) -> Window {
        video
            .window("Chip8", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap()
    }

    pub fn get_canvas(video: &sdl2::VideoSubsystem) -> Canvas<Window> {
        let window = VideoDisplay::init_window(video);
        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
        canvas.set_draw_color(*COLOR_WHITE);
        canvas.clear();
        canvas.present();
        canvas
    }

    /*
    Draw a 1x1 rectangle to represent a pixel. Might not work but lets give it a try.
    */
    pub fn draw_pixel(&mut self, x: usize, y: usize) {
        let rect_x = (x * SCALE as usize) as i32;
        let rect_y = (y * SCALE as usize) as i32;
        let rect_width = SCALE;
        let rect_height = SCALE;
        let rect = sdl2::rect::Rect::new(rect_y, rect_x, rect_width, rect_height);
        let canvas = &mut self.canvas;
        match canvas.fill_rect(rect) {
            Ok(_) => {}
            Err(e) => error!("Could not fill in the rectangle: {}", e),
        }
    }

    /*
    Iterate over the entire VRAM and draw each pixel as a rectangle.
    */
    pub fn draw(&mut self, graphics: &GraphicsMemory) {
        for i in 0..DISPLAY_HEIGHT {
            for j in 0..DISPLAY_WIDTH {
                let pixel = graphics.mem[i][j];
                if pixel != 0 {
                    let canvas = &mut self.canvas;
                    canvas.set_draw_color(*COLOR_RED);
                } else {
                    let canvas = &mut self.canvas;
                    canvas.set_draw_color(*COLOR_GREEN);
                }
                self.draw_pixel(i, j);
            }
        }
        let canvas = &mut self.canvas;
        canvas.present();
    }

    pub fn clear(&mut self, graphics: &mut GraphicsMemory) {
        self.canvas.clear();
        for i in 0..DISPLAY_HEIGHT {
            for j in 0..DISPLAY_WIDTH {
                graphics.mem[i][j] = 0;
            }
        }
    }
}
