use body::Body;
use raylib::prelude::*;
mod body;

fn main() {
    Engine::run();
}

#[derive(Debug)]
pub struct Engine {
    handle: RaylibHandle,
    thread: RaylibThread,
    camera: Camera2D,
}

impl Engine {
    fn run() {
        let (handle, thread) = raylib::init().size(1280, 720).title("Hello, World").build();
        let mut engine = Self {
            handle,
            thread,
            camera: Camera2D {
                offset: Vector2 { x: 0., y: 0. },
                target: Vector2 { x: 0., y: 0. },
                rotation: 0.,
                zoom: 0.5,
            },
        };

        let mut body = Body::new();

        while !engine.handle.window_should_close() {
            let draw = &mut engine.handle.begin_drawing(&engine.thread);
            draw.clear_background(Color::from_hex("192336").unwrap());

            let mut draw2d = draw.begin_mode2D(&engine.camera);

            body.draw(&mut draw2d);
        }
    }
}
