use body::Body;
use ffi::IsKeyDown;
use nalgebra_glm::vec2;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    time::Instant,
};
mod body;
mod plane;
mod polygon;

fn main() {
    let config_file = fs::read_to_string("./config.json")
        .expect("There should be a config file at the root of the project.");

    let config =
        serde_json::from_str(&config_file).expect("The config file should be a JSON file.");

    Engine::run(config);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineConfig {
    setup_file: String,
    display_width: i32,
    display_height: i32,
}

#[derive(Debug)]
pub struct Engine {
    handle: RaylibHandle,
    thread: RaylibThread,
    camera: Camera2D,
}

impl Engine {
    fn run(config: EngineConfig) {
        let (handle, thread) = raylib::init()
            .size(config.display_width, config.display_height)
            .title("Hello, World")
            .build();
        let mut engine = Self {
            handle,
            thread,
            camera: Camera2D {
                offset: Vector2 { x: 0., y: 0. },
                target: Vector2 { x: 0., y: 0. },
                rotation: 0.,
                zoom: 1.,
            },
        };

        let config_file =
            fs::read_to_string(config.setup_file).expect("Could not find the setup file.");

        let mut bodies: Vec<Body> =
            serde_json::from_str(&config_file).expect("Unable to read the setup file");

        for body in bodies.iter_mut() {
            body.construct_collider();
        }

        let mut last_time = Instant::now();

        while !engine.handle.window_should_close() {
            let delta_time = last_time.elapsed().as_secs_f32();

            let mut translation = vec2(0., 0.);

            if unsafe { IsKeyDown(KeyboardKey::KEY_A as i32) } {
                translation = vec2(-1., 0.);
            }

            if unsafe { IsKeyDown(KeyboardKey::KEY_D as i32) } {
                translation = vec2(1., 0.);
            }

            if unsafe { IsKeyDown(KeyboardKey::KEY_W as i32) } {
                translation = vec2(0., -1.);
            }

            if unsafe { IsKeyDown(KeyboardKey::KEY_S as i32) } {
                translation = vec2(0., 1.);
            }

            if translation.magnitude_squared() > 0. {
                bodies[0].position += translation.normalize();
            }

            if unsafe { IsKeyDown(KeyboardKey::KEY_SPACE as i32) } {
                bodies[0].rotation += 0.01;
            }

            for (i, body) in bodies.iter().enumerate() {
                for other in &bodies[i + 1..bodies.len()] {
                    body.check_collision(other, delta_time);
                }
            }

            for body in bodies.iter_mut() {
                body.integrate(delta_time);
            }

            let draw = &mut engine.handle.begin_drawing(&engine.thread);
            draw.clear_background(Color::from_hex("192336").unwrap());

            let mut draw2d = draw.begin_mode2D(engine.camera);

            for body in bodies.iter() {
                body.draw(&mut draw2d);
            }

            last_time = Instant::now();
        }
    }
}
