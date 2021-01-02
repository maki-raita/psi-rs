extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use rand::Rng;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rng = rand::thread_rng();

    let correct_func = |_:bool|true;
    let psi_func = |_:bool|{let mut rng = rand::thread_rng(); rng.gen::<bool>()};

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        };

        let input = rng.gen::<bool>();
        let output = psi_func(input);
        let result = output == correct_func(input);

        ::std::thread::sleep(Duration::from_secs(1));
        println!("input:{}, output:{}, result: {}", input, output, result);
    };
}
