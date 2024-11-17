use nalgebra_glm::{rotation2d, scaling2d, translation2d, vec3};
use nalgebra_glm::{vec2, Mat3x3, Vec2};
use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2D},
};

/// This is a body in the game space
#[derive(Debug)]
pub struct Body {
    points: Vec<Vec2>,
    // TODO: This might need to be its own struct so that it is less of a pain to deal with
    position: Vec2,
    rotation: f32,
    scale: Vec2,
    transform: Mat3x3,
    is_dirty: bool,
}

impl Body {
    pub fn new() -> Self {
        let points = vec![
            Vec2::new(1., 1.),
            Vec2::new(-1., 1.),
            Vec2::new(-1., -1.),
            Vec2::new(1., -1.),
        ];
        Self {
            points,
            position: vec2(1000., 1000.),
            rotation: 0.,
            scale: vec2(100., 100.),
            transform: Mat3x3::identity(),
            is_dirty: true,
        }
    }

    pub fn get_transform(&mut self) -> Mat3x3 {
        if self.is_dirty {
            self.transform =
                translation2d(&self.position) * rotation2d(self.rotation) * scaling2d(&self.scale);
            self.is_dirty = false;
        }
        self.transform
    }

    fn render_points(&mut self) -> Vec<Vector2> {
        let transform = self.get_transform();
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

    pub fn draw(&mut self, handle: &mut RaylibMode2D<RaylibDrawHandle>) {
        let rpoints = self.render_points();

        handle.draw_line_strip(&rpoints, Color::WHITE);
        handle.draw_line_v(&rpoints[0], &rpoints[self.points.len() - 1], Color::WHITE);
    }
}
