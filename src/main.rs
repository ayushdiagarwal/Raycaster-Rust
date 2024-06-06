use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod view;
use view::board_view;

mod object;
use object::player;
use object::rays;

const PI: f64 = 3.14159265358979323846;

// later on, I want this variable to only be in board_view, it's only temp here
const MAP: [[i32; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 0, 1, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

fn main() -> Result<(), String> {
    const CELL_WIDTH: u32 = 64;
    const SCREEN_WIDTH: u32 = CELL_WIDTH * 8;
    const SCREEN_HEIGHT: u32 = CELL_WIDTH * 8;

    let sdl_context: sdl2::Sdl = sdl2::init()?;
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("RAYCASTER", SCREEN_WIDTH, SCREEN_HEIGHT)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let screen_area = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    let clear_color = Color::RGB(64, 192, 255);

    let board_view: board_view::Renderer = board_view::Renderer {
        screen_area: screen_area,
        clear_color: (clear_color),
    };

    let mut player: player::Player = player::Player {
        x: 200.0,
        y: 200.0,
        a: 0.0,
        dx: 0.0,
        dy: 0.0,
    };

    let mut ray: rays::Ray = rays::Ray {
        sx: player.x,
        sy: player.y,
        ra: player.a, // intialize the ray angle to the player angle
        rx: 100.0,    // initial value doesn't matter as I'll be changing it in the loop
        ry: 100.0,
    };

    // background

    // The game loop
    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    // it's better to look for events in the mainloop only as I couldn't get it working in player.rs

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Left => {
                        player.a -= 0.2;
                        player.dx = player.a.cos() * 5.0;
                        player.dy = player.a.sin() * 5.0;
                    }
                    Keycode::Right => {
                        player.a += 0.2;
                        player.dx = player.a.cos() * 5.0;
                        player.dy = player.a.sin() * 5.0;
                    }
                    Keycode::Up => {
                        player.y += player.dy;
                        player.x += player.dx;
                    }
                    Keycode::Down => {
                        player.y -= player.dy;
                        player.x -= player.dx;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // at always, I want rays == players stuff (FOR NOW ONLY)
        ray.ra = player.a;
        ray.sx = player.x;
        ray.sy = player.y;

        // this is rather a hacky way to do it, but I'll fix it later

        // FOR VERTICAL LINES
        let mut map_x = (player.x / 64.0) * 64.0;
        let mut dx: f64 = 0.0;

        if ray.ra.cos() >= 0.0 {
            // looking right
            ray.rx = map_x + CELL_WIDTH as f64;
            dx = CELL_WIDTH as f64;
        }
        if ray.ra.cos() < 0.0 {
            // looking left
            ray.rx = map_x - 0.0001;
            dx = -64 as f64; //Change this to CELL_WIDTH later
        }

        // I don't know how this line works
        // let mut map_y = (player.x - ray.rx) * (ray.ra.tan()) + player.y;

        let mut depth_vert: f64 = (ray.rx - player.x) / ray.ra.cos();
        ray.ry = player.y + depth_vert * ray.ra.sin();

        let delta_depth: f64 = dx / ray.ra.cos();
        let dy: f64 = delta_depth * ray.ra.sin();

        // Now check intersection with map

        let mut dof = 0;

        while dof < 8 {
            let mx = (ray.rx / 64.0).floor() as usize;
            let my = (ray.ry / 64.0).floor() as usize;

            println!("my: {}, mx: {}", my, mx);

            // check for boundary conditions
            if (mx < 8 && my < 8 && mx > 0 && my > 0 && MAP[my][mx] == 1) {
                dof = 8;
            } else {
                ray.rx += dx;
                ray.ry += dy;
                depth_vert += delta_depth;
                dof += 1;
            }
        }

        // Print Stuff
        board_view.render(&mut canvas);
        player.draw(&mut canvas);
        ray.draw(&mut canvas);
        canvas.present();
    }

    Ok(()) // successful execution, doesn't need to return anything, so its empty
}
