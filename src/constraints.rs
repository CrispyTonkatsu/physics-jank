use nalgebra_glm::Vec2;
use raylib::{
    color::Color,
    ffi::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2D},
};

use crate::plane::Plane;

pub trait Constraint {
    fn pre_solve(&mut self, dt: f32);

    fn solve(&mut self, dt: f32);

    fn draw(&self, handle: &mut RaylibMode2D<RaylibDrawHandle>);
}

/// This describes a contact point and the impulses applied to it over several frames
pub struct ContactPoint {
    point: Vec2,
    normal: Vec2,
    penetration: f32,

    to_incident: Vec2,
    to_reference: Vec2,

    accumulated_normal_impulse: f32,
    accumulated_tangent_impulse: f32,
    accumulated_position_bias_impulse: f32,

    mass_normal: Vec2,
    mass_tangent: Vec2,

    bias: f32,

    incident_plane: Plane,
    reference_plane: Plane,
}

impl ContactPoint {
    pub fn new(
        point: Vec2,
        normal: Vec2,
        penetration: f32,
        incident_plane: Plane,
        reference_plane: Plane,
    ) -> Self {
        ContactPoint {
            point,
            normal,
            penetration,
            incident_plane,
            reference_plane,
            ..ContactPoint::default()
        }
    }

    pub fn default() -> Self {
        ContactPoint {
            point: Vec2::default(),
            normal: Vec2::default(),
            to_incident: Vec2::default(),
            to_reference: Vec2::default(),
            penetration: f32::default(),
            accumulated_normal_impulse: f32::default(),
            accumulated_tangent_impulse: f32::default(),
            accumulated_position_bias_impulse: f32::default(),
            mass_normal: Vec2::default(),
            mass_tangent: Vec2::default(),
            bias: f32::default(),
            incident_plane: Plane::default(),
            reference_plane: Plane::default(),
        }
    }

    pub fn draw(
        &self,
        handle: &mut raylib::prelude::RaylibMode2D<raylib::prelude::RaylibDrawHandle>,
    ) {
        handle.draw_circle(self.point.x as i32, self.point.y as i32, 10., Color::PLUM);

        let end_pos = self.point + self.normal * -self.penetration * 100.;
        let end_pos = Vector2 {
            x: end_pos.x,
            y: end_pos.y,
        };

        handle.draw_line_ex(
            Vector2 {
                x: self.point.x,
                y: self.point.y,
            },
            end_pos,
            3.,
            Color::PALEGREEN,
        );

        self.incident_plane.draw(handle, &Color::RED);
        self.reference_plane.draw(handle, &Color::BLUE);
    }
}

// TODO: Implement partial eq so that it can check across frames
impl PartialEq for ContactPoint {
    fn eq(&self, other: &Self) -> bool {
        todo!("Implement the comparison for warm starting")
    }
}
