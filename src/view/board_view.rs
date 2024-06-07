use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Renderer {
    pub screen_area: Rect,
}

use crate::settings::*;

impl Renderer {
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        // background
        // canvas.set_draw_color(Color::RGB(64, 192, 255));
        // canvas.fill_rect(self.screen_area).ok().unwrap();

        // drawing the grid cubes
        const CELLS: i32 = 8;

        for i in 0..MAP.len() {
            for j in 0..MAP[0].len() {
                if MAP[i][j] == 1 {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                } else {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                }
                let x = j * (CELL_SIZE as usize);
                let y = i * (CELL_SIZE as usize);
                let cell = Rect::new(x as i32, y as i32, CELL_SIZE as u32, CELL_SIZE as u32);
                canvas.fill_rect(cell).ok().unwrap();
            }
        }

        // drawing the grid outlines
        for i in 0..CELLS {
            let x = i * (CELL_SIZE as i32);
            let y = i * (CELL_SIZE as i32);
            canvas.set_draw_color(Color::RGB(90, 90, 90));
            canvas
                .draw_line((x, 0), (x, self.screen_area.h))
                .ok()
                .unwrap();
            canvas
                .draw_line((0, y), (self.screen_area.w, y))
                .ok()
                .unwrap();
        }
    }
}
