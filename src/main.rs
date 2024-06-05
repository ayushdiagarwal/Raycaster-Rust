use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() -> Result<(), String> {
    println!("Hello, world!");

    const SCREEN_WIDTH: u32 = 800;
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
    canvas.set_draw_color(clear_color);

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    println!("Mouse moved: x={}, y={}", x, y);
                }

                _ => {} // in the default case, do absolutely nothing
            }
        }

        canvas.fill_rect(screen_area).unwrap();
        canvas.present();
    }

    Ok(()) // successful execution, doesn't need to return anything, so its empty
}
