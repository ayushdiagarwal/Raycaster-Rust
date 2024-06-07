use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::settings::PI;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub a: f64,
    pub dx: f64,
    pub dy: f64,
}

impl Player {
    pub fn check_angle(&mut self) {
        if self.a >= 2.0 * PI {
            self.a -= 2.0 * PI;
        }
        if self.a < 0.0 {
            self.a = 2.0 * PI + self.a;
        }
    }
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let px = self.x as i32;
        let py = self.y as i32;
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        // canvas.fill_rect(Rect::new(px, py, 10, 10)).ok().unwrap();
        draw_circle(canvas, px, py, 10);
    }
}

// found this on the web lol
fn draw_circle(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    cx: i32,
    cy: i32,
    radius: i32,
) {
    let mut x = radius;
    let mut y = 0;
    let mut err = 0;

    while x >= y {
        // Draw horizontal lines
        canvas
            .draw_line(Point::new(cx - x, cy + y), Point::new(cx + x, cy + y))
            .unwrap();
        canvas
            .draw_line(Point::new(cx - x, cy - y), Point::new(cx + x, cy - y))
            .unwrap();

        // Draw vertical lines
        canvas
            .draw_line(Point::new(cx - y, cy + x), Point::new(cx + y, cy + x))
            .unwrap();
        canvas
            .draw_line(Point::new(cx - y, cy - x), Point::new(cx + y, cy - x))
            .unwrap();

        y += 1;
        if err <= 0 {
            err += 2 * y + 1;
        }
        if err > 0 {
            x -= 1;
            err -= 2 * x + 1;
        }
    }
}
