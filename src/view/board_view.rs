use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Renderer {
    pub screen_area: Rect,
    pub clear_color: Color,
}

impl Renderer {
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let map = [
            [1, 1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 1, 0, 1],
            [1, 0, 0, 1, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1],
        ];

        // background
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.screen_area).ok().unwrap();

        // drawing the grid cubes
        const CELLS: i32 = 8;
        let cell_width: u32 = (self.screen_area.w / CELLS) as u32;
        let cell_height: u32 = (self.screen_area.h / CELLS) as u32;

        canvas.fill_rect(Rect::new(0, 0, 100, 100)).ok().unwrap();

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] == 1 {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                } else {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                }
                let x = j * (cell_width as usize);
                let y = i * (cell_height as usize);
                let cell = Rect::new(
                    x.try_into().unwrap(),
                    y.try_into().unwrap(),
                    cell_width.try_into().unwrap(),
                    cell_height.try_into().unwrap(),
                );
                canvas.fill_rect(cell).ok().unwrap();
            }
        }

        // drawing the grid outlines
        for i in 0..CELLS {
            let x = i * (cell_width as i32);
            let y = i * (cell_height as i32);
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
