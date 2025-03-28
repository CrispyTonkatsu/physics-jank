use body::Body;
use catppuccin::ColorName;
use collision_constraint::CollisionConstraint;
use constraints::Constraint;
use nalgebra_glm::{vec2, Vec2};
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::HashMap,
    fs::{self},
    rc::Rc,
};

mod body;
mod collision_constraint;
mod constraints;
mod contact_point;
mod plane;
mod polygon;

mod color;

// NOTE: Main will not be working for a bit given certain changes will be done to the structure of
// the collision data generated
//
// NOTE: Also, remove all the notes when you are sure it works

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
    iteration_count: usize,
}

pub struct Engine {
    handle: RaylibHandle,
    thread: RaylibThread,
    camera: Camera2D,

    bodies: Vec<Rc<RefCell<Body>>>,

    general_constraints: Vec<Box<dyn Constraint>>,
    collision_map: HashMap<(usize, usize), CollisionConstraint>,
    iteration_count: usize,
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
            general_constraints: vec![],
            collision_map: HashMap::default(),
            iteration_count: config.iteration_count,
        };

        engine.load_simulation(config);

        while !engine.handle.window_should_close() {
            let delta_time = engine.handle.get_frame_time();

            engine.check_collisions(delta_time);
            engine.test_controller();
            engine.resolve_collisions(delta_time);
            engine.integrate(delta_time);
            engine.draw();
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
            for (j, other_body_cell) in self.bodies[i + 1..length].iter().enumerate() {
                let body = (**body_cell).borrow();
                let other_body = (**other_body_cell).borrow();
                let sat_output = body.check_collision(&other_body, dt);

                if let Some((is_reference, normal, ..)) = sat_output {
                    let (incident_body, reference_body) = if is_reference {
                        (other_body_cell, body_cell)
                    } else {
                        (body_cell, other_body_cell)
                    };

                    let new_manifold = CollisionConstraint::generate_manifold(
                        normal,
                        &incident_body.borrow(),
                        &reference_body.borrow(),
                    );

                    match self.collision_map.get_mut(&(i, j)) {
                        Some(constraint) => {
                            constraint.update_manifold(new_manifold);
                        }
                        None => {
                            self.collision_map.insert(
                                (i, j),
                                CollisionConstraint::new(
                                    new_manifold,
                                    incident_body.clone(),
                                    reference_body.clone(),
                                ),
                            );
                        }
                    }
                } else {
                    // Remove collisions that did not happen
                    self.collision_map.remove(&(i, j));
                }
            }
        }
    }

    fn resolve_collisions(&mut self, dt: f32) {
        let inv_dt = 1. / dt;

        for general_constraint in self.general_constraints.iter_mut() {
            general_constraint.pre_solve(inv_dt);
        }

        for (.., contact_constraint) in self.collision_map.iter_mut() {
            contact_constraint.pre_solve(inv_dt);
        }

        // Solving the constraints
        for _ in 0..self.iteration_count {
            for constraint in self.general_constraints.iter_mut() {
                constraint.solve();
            }

            for (.., constraint) in self.collision_map.iter_mut() {
                constraint.solve();
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
        let draw = &mut self.handle.begin_drawing(&self.thread);
        draw.clear_background(color::get(ColorName::Base));

        let mut draw2d = draw.begin_mode2D(self.camera);

        for body in self.bodies.iter() {
            body.borrow().draw(&mut draw2d);
        }

        for (.., constraint) in self.collision_map.iter_mut() {
            constraint.draw(&mut draw2d);
        }
    }

    fn test_controller(&mut self) {
        let handle = &self.handle;
        let mut body = (*self.bodies[0]).borrow_mut();

        let mut impulse = vec2(0., 0.);

        if handle.is_key_down(KeyboardKey::KEY_W) {
            impulse += Vec2::new(0., -1.);
        }

        if handle.is_key_down(KeyboardKey::KEY_S) {
            impulse += Vec2::new(0., 1.);
        }

        if handle.is_key_down(KeyboardKey::KEY_A) {
            impulse += Vec2::new(-1., 0.);
        }

        if handle.is_key_down(KeyboardKey::KEY_D) {
            impulse += Vec2::new(1., 0.);
        }

        if impulse.magnitude() > 0. {
            body.apply_impulse(impulse.normalize());
        }

        let mut impulse = 0.;

        if handle.is_key_down(KeyboardKey::KEY_J) {
            impulse += 0.05;
        }

        if handle.is_key_down(KeyboardKey::KEY_K) {
            impulse += -0.05;
        }

        body.apply_angular_impulse(impulse);
    }
}
