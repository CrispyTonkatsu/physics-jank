use std::fs;

use nalgebra_glm::{rotation2d, scaling2d, translation2d, vec2};
use nalgebra_glm::{Mat3x3, Vec2};
use raylib::prelude::{RaylibDrawHandle, RaylibMode2D};
use serde::{Deserialize, Serialize};

use crate::polygon::Polygon;

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    // Non-Physics Variables
    position: Vec2,
    rotation: f32,
    scale: Vec2,
    collider_file: String,
    // Body Properties (Probably turn the f32s into some sort of properties struct)
    collider: Option<Polygon>,
    mass: f32,
    inertia: f32,
    restitution: f32,
    friction: f32,
    #[serde(default)]
    is_static: bool,
    // Linear Runtime Physics Variables
    #[serde(default)]
    velocity: Vec2,
    #[serde(default)]
    net_force: Vec2,
    // Angular Runtime Physics Variables
    #[serde(default)]
    angular_velocity: f32,
    #[serde(default)]
    moment: f32,
}

impl Body {
    pub fn check_collision(&self, other: &Body, dt: f32) {
        if let (Some(collider), Some(other_collider)) = (&self.collider, &other.collider) {
            collider
                .get_in_world(&self.get_transform())
                .check_collision(&other_collider.get_in_world(&other.get_transform()), dt);
        }
    }

    pub fn calc_loads(&mut self) {
        self.net_force = vec2(0., 9.81) * 10000.;
        self.moment = 0.;
    }

    pub fn integrate(&mut self, dt: f32) {
        if self.is_static {
            return;
        }

        self.calc_loads();

        let acceleration = self.net_force / self.mass;
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;

        let angular_acceleration = self.moment / self.inertia;
        self.angular_velocity += angular_acceleration * dt;
        self.rotation += self.angular_velocity * dt;
    }

    pub fn construct_collider(&mut self) {
        let collider_file =
            fs::read_to_string("assets/colliders/".to_string() + &self.collider_file)
                .expect("Could not find collider file.");

        let collider = serde_json::from_str(&collider_file);

        match collider {
            Ok(collider) => self.collider = collider,
            Err(error) => println!("Error reading collider file: {}", error),
        }
    }

    pub fn get_transform(&self) -> Mat3x3 {
        translation2d(&self.position) * rotation2d(self.rotation) * scaling2d(&self.scale)
    }

    pub fn draw(&self, handle: &mut RaylibMode2D<RaylibDrawHandle>) {
        if let Some(collider) = &self.collider {
            collider.draw(&self.get_transform(), handle);
        }
    }
}
