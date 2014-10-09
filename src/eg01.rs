extern crate sdl2;
extern crate native;

use sdl2::video::{Window, PosCentered, OPENGL};
use sdl2::timer::{delay};

fn main() {
    // start sdl2

    sdl2::init(sdl2::INIT_VIDEO);

    // Create a window

    let window = match Window::new("eg01", PosCentered, PosCentered, 640, 480, OPENGL) {
        Ok(window) => window,
        Err(err)   => fail!("failed to create window: {}", err)
    };

    // Display the window for 3 seconds

    window.show();
    delay(3000);

    // then quit

    sdl2::quit();
}

