use std::fs;

use nalgebra_glm::{rotation2d, scaling2d, translation2d};
use nalgebra_glm::{Mat3x3, Vec2};
use raylib::prelude::{RaylibDrawHandle, RaylibMode2D};
use serde::{Deserialize, Serialize};

use crate::polygon::Polygon;

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    position: Vec2,
    rotation: f32,
    scale: Vec2,
    collider: Option<Polygon>,
    collider_file: String,
}

impl Body {
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
