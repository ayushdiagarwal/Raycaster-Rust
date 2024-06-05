use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub a: f64,
    pub dx: f64,
    pub dy: f64,
}

impl Player {
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let px = self.x as i32;
        let py = self.y as i32;
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(Rect::new(px, py, 10, 10)).ok().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas
            .draw_line(
                (px, py),
                (
                    (self.x + self.a.cos() * 20.0) as i32,
                    (self.y + self.a.sin() * 20.0) as i32,
                ),
            )
            .ok()
            .unwrap();
    }
}
