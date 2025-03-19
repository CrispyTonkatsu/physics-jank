use body::Body;
use collision_info::CollisionInfo;
use constraints::Constraint;
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
mod constraints;
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
    bodies: Vec<Rc<RefCell<Body>>>,
    collisions: Vec<CollisionInfo>,
}

impl Engine {
    fn run(config: EngineConfig) {
        let (handle, thread) = raylib::init()
            .resizable()
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
        for (i, body_cell) in self.bodies[0..length - 1].iter().enumerate() {
            for other_body_cell in &self.bodies[i + 1..length] {
                let body = (**body_cell).borrow_mut();
                let other_body = (**other_body_cell).borrow_mut();
                let sat_output = body.check_collision(&other_body, dt);

                if let Some((is_reference, normal, penetration)) = sat_output {
                    let (incident_body, reference_body) = if is_reference {
                        (body_cell, other_body_cell)
                    } else {
                        (other_body_cell, body_cell)
                    };

                    self.collisions.push(CollisionInfo::new(
                        normal,
                        penetration,
                        incident_body.clone(),
                        reference_body.clone(),
                    ));
                }
            }
        }
    }

    // TODO: implement the collision resolution algorithm here (read the docs you bookmarked)
    fn resolve_collisions(&mut self, dt: f32) {
        // Generating constraints where appropriate
        let mut constraints: Vec<Box<dyn Constraint>> = vec![];

        for collision in self.collisions.iter_mut() {
            constraints.push(collision.generate_constraint());
        }
        self.collisions.clear();

        // Solving the constraints
        // TODO: Make max iterations a field that can be edited in the engine config file
        let iteration_max = 10000;
        if iteration_max <= 0 {
            // Guard against 0 iterations (shouldn't be the value to use but me when I don't crash,
            // its lovely)
            dbg!("Iteration maximum shuold be > 0");
            return;
        }

        let solver_dt = dt / (iteration_max as f32);
        for _ in 0..iteration_max {
            for constraint in constraints.iter_mut() {
                constraint.solve(solver_dt);
            }
        }
    }

    fn integrate(&mut self, dt: f32) {
        for body in self.bodies.iter_mut() {
            let mut body = (**body).borrow_mut();
            body.integrate(dt);
        }
    }

    fn draw(&mut self) {
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
