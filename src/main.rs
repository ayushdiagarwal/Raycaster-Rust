use object::player;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod view;
use view::board_view;

mod object;
use object::rays;

mod settings;
use settings::*; // importing all constants

fn main() -> Result<(), String> {
    let sdl_context: sdl2::Sdl = sdl2::init()?;
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("RAYCASTER", SCREEN_WIDTH * 2, SCREEN_HEIGHT)
        .build()
        .unwrap();

    let mut canvas_2d = window.into_canvas().build().unwrap();

    let screen_area = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);

    let board_view: board_view::Renderer = board_view::Renderer {
        screen_area, // shorthand intialization
    };

    // Creating a player's instance
    let mut player: player::Player = player::Player {
        x: 200.0,
        y: 200.0,
        a: 0.0,
        // at angle 0, it faces exactly right
        dx: 5.0,
        dy: 0.0,
    };

    // The game loop
    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    let mut rays: Vec<rays::Ray> = Vec::new();
    let mut player_moved = true;

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
                        player.dx = player.a.cos() * 10.0;
                        player.dy = player.a.sin() * 10.0;
                        player_moved = true;
                    }
                    Keycode::Right => {
                        player.a += 0.2;
                        player.dx = player.a.cos() * 10.0;
                        player.dy = player.a.sin() * 10.0;
                        player_moved = true;
                    }
                    Keycode::Up => {
                        // collision detection
                        let y1 = ((player.y + player.dy) / 64.0).floor() as usize;
                        let x1 = ((player.x + player.dx) / 64.0).floor() as usize;

                        if MAP[y1][x1] != 1 {
                            player.y += player.dy;
                            player.x += player.dx;
                            player_moved = true;
                        }
                    }
                    Keycode::Down => {
                        // collision detection
                        let y1 = ((player.y - player.dy) / 64.0).floor() as usize;
                        let x1 = ((player.x - player.dx) / 64.0).floor() as usize;

                        if MAP[y1][x1] != 1 {
                            player.y -= player.dy;
                            player.x -= player.dx;
                            player_moved = true;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        player.check_angle(); // keeps angle btw 0 and 2PI

        if player_moved {
            rays.clear();

            // for multiple rays
            for i in 0..NUM_RAYS {
                let mut ray: rays::Ray = rays::Ray {
                    sx: player.x,
                    sy: player.y,
                    // Initial values of these don't matter
                    ra: 0.0,
                    rx: 0.0,
                    ry: 0.0,
                    ri: i,
                    proj_height: 0.0,
                    depth: 0.0,
                };

                rays::Ray::handle_events(&mut ray, &mut player, &mut rays, i);
            }
            player_moved = false;
        }

        // the thing wrong here is that if i don't clear the screen with the background color, some pretty nasty stuff happens,
        // for which I have absolutely no idea why it happens

        // Render Stuff
        canvas_2d.set_draw_color(Color::RGB(30, 30, 30)); // background color
        canvas_2d
            .fill_rect(Rect::new(0, 0, SCREEN_WIDTH * 2, SCREEN_HEIGHT))
            .ok()
            .unwrap();
        board_view.render(&mut canvas_2d);

        for ray in &mut rays {
            ray.draw_3D(&mut canvas_2d);
            ray.draw_2D(&mut canvas_2d);
        }
        player.draw(&mut canvas_2d);

        canvas_2d.present();
    }

    Ok(()) // successful execution, doesn't need to return anything, so its empty
}
