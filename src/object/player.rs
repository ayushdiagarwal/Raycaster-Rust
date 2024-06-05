// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
// use sdl2::EventPump;

pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Player {
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas
            .fill_rect(Rect::new(self.x, self.y, 50, 50))
            .ok()
            .unwrap();
    }
}
