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
    /// This function will perform the SAT test with both shapes and return the data of the
    /// collision. This will be the raw data such that it can be fit into a struct afterwards.
    /// Return data = (caller_is_reference_face, normal, penetration)
    pub fn check_collision(&self, other: &Polygon, _dt: f32) -> Option<(bool, Vec2, f32)> {
        let query = self.query_faces(other);
        if query.1 > 0. {
            return None;
        }

        let query_other = other.query_faces(self);
        if query_other.1 > 0. {
            return None;
        }

        if query.1 < query_other.1 {
            Some((
                true,
                self.get_plane(query.0).get_normal().normalize(),
                query.1,
            ))
        } else {
            Some((
                false,
                other.get_plane(query_other.0).get_normal().normalize(),
                query_other.1,
            ))
        }
    }

    pub fn query_faces(&self, other: &Polygon) -> (usize, f32) {
        let mut max_distance = -f32::INFINITY;
        let mut max_index = 0;

        for i in 0..self.points.len() {
            let plane = self.get_plane(i);
            let support = other.map_support(-plane.get_normal());
            let distance = plane.distance_to(&support);

            if distance > max_distance {
                max_distance = distance;
                max_index = i;
            }
        }
        (max_index, max_distance)
    }

    // This could work with circles if you use the support point properly
    pub fn map_support(&self, direction: Vec2) -> Vec2 {
        self.points
            .iter()
            .max_by(|a, b| {
                (a.dot(&direction).partial_cmp(&b.dot(&direction)))
                    .expect("Vector dots resulted in inf (check the math)")
            })
            .copied()
            .unwrap_or(vec2(0.0, 0.0))
    }

    pub fn get_significant_face_with_index(&self, normal: Vec2) -> (Plane, usize) {
        let support_point = self.map_support(normal);
        let mut max_plane_dot: f32 = 0.;
        let mut max_plane_index = 0;

        for i in 0..self.points.len() {
            let plane = self.get_plane(i);

            if plane.is_made_of(&support_point) && plane.get_normal().dot(&normal) > max_plane_dot {
                max_plane_dot = plane.get_normal().dot(&normal);
                max_plane_index = i;
            }
        }

        (self.get_plane(max_plane_index), max_plane_index)
    }

    pub fn get_significant_face(&self, normal: Vec2) -> Plane {
        self.get_significant_face_with_index(normal).0
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

    pub fn draw(
        &self,
        body_transform: &Mat3x3,
        handle: &mut RaylibMode2D<RaylibDrawHandle>,
        color: Color,
    ) {
        let rpoints = self.render_points(body_transform);

        handle.draw_line_strip(&rpoints, color);
        handle.draw_line_v(rpoints[0], rpoints[self.points.len() - 1], color);
    }

    pub fn point_count(&self) -> usize {
        self.points.len()
    }
}
