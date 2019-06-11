#[macro_use]
extern crate glium;
extern crate image;

mod graphics;

use glium::glutin;
use graphics::console::*;
use graphics::tileset::*;
use std::time::Instant;

const SIZE: (u32, u32) = (80, 80);
const NAME: &str = "gguurr's game";


fn update(root: &mut Root, closed: &mut bool, delta: f64) {
    root.events_loop.poll_events(|ev| match ev {
        glutin::Event::WindowEvent { event, .. } => match event {
            glutin::WindowEvent::CloseRequested => *closed = true,
            _ => (),
        },
        _ => (),
    });
}

fn draw(root: &mut Root, delta: f64) {
    root.clear();
    root.draw();
}


fn main() {
    let mut closed = false;

    let ts = TileSet::new("./tileset.png", (10, 10), (0, 0));
    let mut root = Root::new(ts, SIZE, NAME);
    let mut n = Instant::now();
    while !closed {
        let mut delta = Instant::now().duration_since(n).as_micros() as f64  / 10.0;
        if delta == 0.0 {
            delta = 0.0001;
        }
        update(&mut root, &mut closed, delta);
        draw(&mut root, delta);
        n = Instant::now();
    }
}
