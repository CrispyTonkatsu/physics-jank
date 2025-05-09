use std::fs;

use nalgebra_glm::{rotation2d, scaling2d, translation2d, vec2};
use nalgebra_glm::{Mat3x3, Vec2, Vec3};
use raylib::color::Color;
use raylib::prelude::{RaylibDrawHandle, RaylibMode2D};
use serde::{Deserialize, Serialize};

use crate::color;
use crate::polygon::Polygon;

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    // Non-Physics Variables
    pub position: Vec2,
    pub rotation: f32,
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
    center_of_gravity: Vec2,
    #[serde(default)]
    angular_velocity: f32,
    #[serde(default)]
    moment: f32,
}

impl Body {
    pub fn check_collision(&self, other: &Body, dt: f32) -> Option<(bool, Vec2, f32)> {
        let collider = self.collider.as_ref()?;
        let other_collider = other.collider.as_ref()?;

        let collider = collider.get_in_world(&self.get_transform());
        let other_collider = other_collider.get_in_world(&other.get_transform());

        collider.check_collision(&other_collider, dt)
    }

    pub fn integrate(&mut self, dt: f32) {
        if self.is_static {
            return;
        }

        // TODO: Make gravity changeable
        let gravity = vec2(0., 9.81) * 0.;

        let acceleration = (1. / self.mass) * self.net_force + gravity;
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
            collider.draw(
                &self.get_transform(),
                handle,
                color::get(catppuccin::ColorName::Flamingo),
            );
        }
    }

    pub fn is_static(&self) -> bool {
        self.is_static
    }

    pub fn apply_impulse(&mut self, impulse: Vec2) {
        self.velocity += impulse * self.inv_mass();
    }

    pub fn apply_angular_impulse(&mut self, impulse: f32) {
        self.angular_velocity += impulse * self.inv_inertia();
    }

    pub fn collider(&self) -> Option<&Polygon> {
        self.collider.as_ref()
    }

    pub fn collider_mut(&mut self) -> &mut Option<Polygon> {
        &mut self.collider
    }

    pub fn collider_in_world(&self) -> Option<Polygon> {
        self.collider()
            .map(|collider| collider.get_in_world(&self.get_transform()))
    }

    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn angular_velocity(&self) -> f32 {
        self.angular_velocity
    }

    pub fn center_of_gravity(&self) -> Vec2 {
        // NOTE: this only works for center of gravity at Vec2::zero, the rotations need to be done
        // about the center of gravity for it to be nice with this
        (self.get_transform() * Vec3::new(self.center_of_gravity.x, self.center_of_gravity.y, 1.))
            .xy()
    }

    pub fn mass(&self) -> f32 {
        if !self.is_static() {
            self.mass
        } else {
            0.
        }
    }

    pub fn inertia(&self) -> f32 {
        if !self.is_static() {
            self.inertia
        } else {
            0.
        }
    }

    pub fn inv_mass(&self) -> f32 {
        if !self.is_static() {
            1. / self.mass
        } else {
            0.
        }
    }

    pub fn inv_inertia(&self) -> f32 {
        if !self.is_static() {
            1. / self.inertia
        } else {
            0.
        }
    }
}
