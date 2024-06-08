use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::object::player::Player;

use crate::settings::*;

// is there a way to lower the amount of parameters?
pub struct Ray {
    pub sx: f64, // starting position which is equal to player's
    pub sy: f64, // starting y position
    pub ra: f64,
    pub rx: f64,
    pub ry: f64,
    pub ri: u32, // ray index
    pub proj_height: f64,
    pub depth: f64,
}

impl Ray {
    pub fn handle_events(&mut self, player: &mut Player, rays: &mut Vec<Ray>, i: u32) {
        // adding small amount so it doesn't equal to zero
        self.ra = player.a - HALF_FOV + 0.0001 + i as f64 * DELTA_ANGLE;

        // keeps angle between 0 and 2PI
        if self.ra >= 2.0 * PI {
            self.ra -= 2.0 * PI;
        }
        if self.ra < 0.0 {
            self.ra = 2.0 * PI + self.ra;
        }

        // starting position of ray is player's position
        self.sx = player.x;
        self.sy = player.y;

        // FOR VERTICAL LINES
        let map_x = (player.x / 64.0).floor() * 64.0;
        let mut dx: f64 = 0.0;
        let mut x_vert: f64 = 0.0;

        if self.ra.cos() >= 0.0 {
            // looking right
            x_vert = map_x + CELL_SIZE as f64;
            dx = CELL_SIZE as f64;
        }
        if self.ra.cos() < 0.0 {
            // looking left
            x_vert = map_x - 0.0001;
            dx = -64 as f64; // Change this to CELL_SIZE later
        }

        let mut depth_vert = (x_vert - player.x) / self.ra.cos();
        let mut y_vert = player.y + depth_vert * self.ra.sin();

        let delta_depth = dx / self.ra.cos();
        let dy = delta_depth * self.ra.sin();

        // Checking intersection with map

        let mut dof_v = 0;

        while dof_v < 8 {
            let mx = (x_vert / 64.0).floor() as usize;
            let my = (y_vert / 64.0).floor() as usize;
            // check for boundary conditions
            #[allow(unused_comparisons)]
            if mx < 8 && my < 8 && mx >= 0 && my >= 0 && MAP[my][mx] == 1 {
                dof_v = 8;
            } else {
                x_vert += dx;
                y_vert += dy;
                depth_vert += delta_depth;
                dof_v += 1;
            }
        }

        // FOR HORIZONTAL LINES
        let map_y = (player.y / 64.0).floor() * 64.0;
        let dy: f64;
        let mut y_horz: f64;

        if self.ra < PI {
            // looking down
            y_horz = map_y + CELL_SIZE as f64;
            dy = CELL_SIZE as f64;
        } else {
            // looking up
            y_horz = map_y - 0.0001;
            dy = -64 as f64; //Change this to CELL_SIZE later
        }

        let mut depth_horz: f64 = (y_horz - player.y) / self.ra.sin();
        let mut x_horz = player.x + depth_horz * self.ra.cos();

        let delta_depth_hor: f64 = dy / self.ra.sin();
        let dx: f64 = delta_depth_hor * self.ra.cos();

        // Checking intersection with map

        let mut dof_h = 0;

        while dof_h < 8 {
            let mx = (x_horz / 64.0).floor() as usize;
            let my = (y_horz / 64.0).floor() as usize;

            #[allow(unused_comparisons)]
            if mx < 8 && my < 8 && mx >= 0 && my >= 0 && MAP[my][mx] == 1 {
                dof_h = 8;
            } else {
                x_horz += dx;
                y_horz += dy;
                depth_horz += delta_depth_hor;
                dof_h += 1;
            }
        }

        // Checking which line is closer

        let mut depth: f64;

        if depth_vert < depth_horz {
            self.rx = x_vert;
            self.ry = y_vert;
            depth = depth_vert;
        } else {
            self.rx = x_horz;
            self.ry = y_horz;
            depth = depth_horz;
        }

        // removing the fishbowl effect
        depth *= (player.a - self.ra).cos(); // getting the distance between player and wall

        let tan_half_fov = (HALF_FOV).tan();
        let screen_dist = HALF_WIDTH as f64 / tan_half_fov;

        let proj_height = CELL_SIZE as f64 * screen_dist / (depth + 0.00001); // to avoid division by zero

        rays.push(Ray {
            sx: self.sx,
            sy: self.sy,
            ra: self.ra,
            rx: self.rx,
            ry: self.ry,
            ri: i,
            // shorthand intialization
            proj_height,
            depth,
        });
    }
    #[allow(non_snake_case)]
    pub fn draw_2D(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(252, 186, 3));
        canvas
            .draw_line(
                Point::new(self.sx as i32, self.sy as i32),
                Point::new(self.rx as i32, self.ry as i32),
            )
            .ok()
            .unwrap();
    }
    #[allow(non_snake_case)]
    pub fn draw_3D(&mut self, canvas: &mut Canvas<Window>) {
        let color = (255.0 / (1.0 + self.depth * 0.006)) as u8;
        canvas.set_draw_color(Color::RGB(color, color, color));
        canvas
            .fill_rect(Rect::new(
                (SCREEN_WIDTH + (self.ri * SCALE)) as i32, // x coordinate
                (HALF_HEIGHT as f64 - (self.proj_height / 2.0).floor()) as i32, // y coordinate
                SCALE,                                     // width of each ray
                self.proj_height as u32,
            ))
            .ok()
            .unwrap();
    }
}
