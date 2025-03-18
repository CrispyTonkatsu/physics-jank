use body::Body;
use collision_info::CollisionInfo;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    fs::{self},
    rc::Rc,
    time::Instant,
};
mod body;
mod collision_info;
mod plane;
mod polygon;
use macroquad::prelude::*;

// HACK: Left off here, migrating to macroquad for nicer windows and even less thinking about
// rendering (Look at how simple the framework is, me when proc macros are crazy)
#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

fn old_main() {
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
    bodies: Vec<Rc<RefCell<Body>>>,
    collisions: Vec<CollisionInfo>,
}

impl Engine {
    fn run(config: EngineConfig) {
        let (handle, thread) = raylib::init()
            .size(config.display_width, config.display_height)
            .title("Physics Jank")
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
            bodies: vec![],
            collisions: vec![],
        };

        engine.load_simulation(config);

        let mut last_time = Instant::now();

        while !engine.handle.window_should_close() {
            let delta_time = last_time.elapsed().as_secs_f32();

            engine.check_collisions(delta_time);
            engine.resolve_collisions(delta_time);
            engine.integrate(delta_time);
            engine.draw();

            last_time = Instant::now();
        }
    }

    fn load_simulation(&mut self, config: EngineConfig) {
        let config_file =
            fs::read_to_string(config.setup_file).expect("Could not find the setup file.");

        let mut bodies: Vec<Body> =
            serde_json::from_str(&config_file).expect("Unable to read the setup file");

        for body in bodies.iter_mut() {
            body.borrow_mut().construct_collider();
        }

        self.bodies = bodies
            .into_iter()
            .map(|x| Rc::new(RefCell::new(x)))
            .collect();
    }

    fn check_collisions(&mut self, dt: f32) {
        let length = self.bodies.len();
        for (i, body) in self.bodies[0..length - 1].iter().enumerate() {
            for other_body in &self.bodies[i + 1..length] {
                let sat_output;
                {
                    let body = (**body).borrow_mut();
                    let other_body = (**other_body).borrow_mut();

                    sat_output = body.check_collision(&other_body, dt);
                }

                if let Some(sat_output) = sat_output {
                    let normal = sat_output.normalize();
                    let penetration = sat_output.magnitude();
                    self.collisions.push(CollisionInfo::new(
                        normal,
                        penetration,
                        body.clone(),
                        other_body.clone(),
                    ));
                }
            }
        }
    }

    fn resolve_collisions(&mut self, _dt: f32) {
        // TODO: implement the collision resolution algorithm here (read the docs you bookmarked)
        for _collision in self.collisions.iter() {}
    }

    fn integrate(&mut self, dt: f32) {
        for body in self.bodies.iter_mut() {
            let mut body = (**body).borrow_mut();
            body.integrate(dt);
        }
    }

    fn draw(&mut self) {
        // TODO: Make the world scale a little bit better so that values don't need to be massive

        // TODO: Make the colors nicer to look at so that I don't have a stroke when debugging this
        // for 5 hours straight (probs use Rose Pine colors for the background and shapes).
        let draw = &mut self.handle.begin_drawing(&self.thread);
        draw.clear_background(Color::from_hex("192336").unwrap());

        let mut draw2d = draw.begin_mode2D(self.camera);

        for body in self.bodies.iter() {
            body.borrow().draw(&mut draw2d);
        }
    }
}
