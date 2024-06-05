use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

mod view;
use view::board_view;

fn main() -> Result<(), String> {
    println!("Hello, world!");

    const SCREEN_WIDTH: u32 = 600;
    const SCREEN_HEIGHT: u32 = 600;

    let sdl_context: sdl2::Sdl = sdl2::init()?;
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("hello", SCREEN_WIDTH, SCREEN_HEIGHT)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let screen_area = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    let clear_color = Color::RGB(64, 192, 255);

    let board_view: board_view::Renderer = board_view::Renderer {
        screen_area: screen_area,
        clear_color: (clear_color),
    };

    // background
    // canvas.set_draw_color(clear_color);

    // The game loop
    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }

                _ => {} // in the default case, do absolutely nothing
            }
        }

        // canvas.fill_rect(screen_area).ok().unwrap();
        board_view.render(&mut canvas);
        canvas.present();
    }

    Ok(()) // successful execution, doesn't need to return anything, so its empty
}
