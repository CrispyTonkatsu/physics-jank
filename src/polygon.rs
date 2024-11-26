use core::f32;

use nalgebra_glm::{rotation2d, scaling2d, translation2d, vec2, vec3, Mat3x3, Vec2};
use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2D},
};
use serde::{Deserialize, Serialize};

use crate::plane::Plane;

#[derive(Debug, Serialize, Deserialize)]
pub struct Polygon {
    points: Vec<Vec2>,
    position: Vec2,
    rotation: f32,
    scale: Vec2,
}

impl Polygon {
    pub fn check_collision(&self, other: &Polygon, _dt: f32) -> bool {
        let query = self.query_faces(other);
        let query_other = other.query_faces(self);
        println!("{}, {}", query.0, query.1);
        query.1 < 0. && query_other.1 < 0.
    }

    pub fn query_faces(&self, other: &Polygon) -> (usize, f32) {
        // TODO: Turn this into iterator stuff, it probably looks prettier
        let mut max_distance = f32::INFINITY;
        let mut max_index = 0;
        for i in 0..self.points.len() {
            let plane = self.get_plane(i);
            let support = other.map_support(-plane.get_normal());
            let distance = plane.distance_to(&support);

            if distance < max_distance {
                max_distance = distance;
                max_index = i;
            }
        }
        (max_index, max_distance)
    }

    pub fn map_support(&self, direction: Vec2) -> Vec2 {
        let search = self.points.iter().max_by(|a, b| {
            (a.dot(&direction).partial_cmp(&b.dot(&direction)))
                .expect("Vector dots resulted in inf (check the math)")
        });
        match search {
            Some(x) => *x,
            None => {
                println!("Empty point vector");
                vec2(0., 0.)
            }
        }
    }

    pub fn get_plane(&self, index: usize) -> Plane {
        let wrapped_index = (index + 1) % self.points.len();
        Plane::new(self.points[index], self.points[wrapped_index])
    }

    pub fn get_transform(&self) -> Mat3x3 {
        translation2d(&self.position) * rotation2d(self.rotation) * scaling2d(&self.scale)
    }

    pub fn get_in_world(&self, body_transform: &Mat3x3) -> Self {
        Polygon {
            points: self.global_points(body_transform),
            ..*self
        }
    }

    pub fn global_points(&self, body_transform: &Mat3x3) -> Vec<Vec2> {
        let transform = body_transform * self.get_transform();
        self.points
            .iter()
            .map(|point| (transform * vec3(point.x, point.y, 1.)).xy())
            .collect()
    }

    fn render_points(&self, body_transform: &Mat3x3) -> Vec<Vector2> {
        self.global_points(body_transform)
            .iter()
            .map(|point| Vector2 {
                x: point.x,
                y: point.y,
            })
            .collect()
    }

    pub fn draw(&self, body_transform: &Mat3x3, handle: &mut RaylibMode2D<RaylibDrawHandle>) {
        let rpoints = self.render_points(body_transform);

        handle.draw_line_strip(&rpoints, Color::WHITE);
        handle.draw_line_v(rpoints[0], rpoints[self.points.len() - 1], Color::WHITE);
    }
}
