use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Ray {
    pub sx: f64,
    pub sy: f64,
    pub ra: f64,
    pub rx: f64,
    pub ry: f64,
}

impl Ray {
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        // self.rx = player.rx;

        // do nothing
        canvas.set_draw_color(Color::RGB(252, 186, 3));
        canvas
            .draw_line(
                Point::new(self.sx as i32, self.sy as i32),
                Point::new(self.rx as i32, self.ry as i32),
            )
            .ok()
            .unwrap();
    }
}
