use nalgebra_glm::{rotation2d, scaling2d, translation2d, vec3, Mat3x3, Vec2};
use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2D},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Polygon {
    points: Vec<Vec2>,
    position: Vec2,
    rotation: f32,
    scale: Vec2,
}

impl Polygon {
    pub fn get_transform(&self) -> Mat3x3 {
        translation2d(&self.position) * rotation2d(self.rotation) * scaling2d(&self.scale)
    }

    fn render_points(&self, body_transform: &Mat3x3) -> Vec<Vector2> {
        let transform = body_transform * self.get_transform();
        self.points
            .iter()
            .map(|point| {
                let tpoint = transform * vec3(point.x, point.y, 1.);
                Vector2 {
                    x: tpoint.x,
                    y: tpoint.y,
                }
            })
            .collect()
    }

    pub fn draw(&self, body_transform: &Mat3x3, handle: &mut RaylibMode2D<RaylibDrawHandle>) {
        let rpoints = self.render_points(body_transform);

        handle.draw_line_strip(&rpoints, Color::WHITE);
        handle.draw_line_v(rpoints[0], rpoints[self.points.len() - 1], Color::WHITE);
    }
}
