use nalgebra_glm::Vec2;
use raylib::{color::Color, ffi::Vector2, prelude::RaylibDraw};

use crate::plane::Plane;

/// This is the way that each contact point will identify itself with.
#[derive(Clone)]
pub struct ContactID {
    edge_a_is_reference: bool,
    edge_a: usize,
    edge_b_is_reference: bool,
    edge_b: usize,
}

impl ContactID {
    pub fn new(
        edge_a_is_reference: bool,
        edge_a: usize,
        edge_b_is_reference: bool,
        edge_b: usize,
    ) -> Self {
        Self {
            edge_a_is_reference,
            edge_a,
            edge_b_is_reference,
            edge_b,
        }
    }

    /// The first edge cannot be set as it will always be the reference
    pub fn set_second_edge(&mut self, is_reference: bool, index: usize) {
        self.edge_b_is_reference = is_reference;
        self.edge_b = index;
    }
}

impl PartialEq for ContactID {
    fn eq(&self, other: &Self) -> bool {
        self.edge_a_is_reference == other.edge_a_is_reference
            && self.edge_a == other.edge_a
            && self.edge_b_is_reference == other.edge_b_is_reference
            && self.edge_b == other.edge_b
    }
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

    // HACK: Left off here, building the feature IDs for the point and moving plane data to the
    // constraint itself as the point doesnt use it
    id: ContactID,

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
        id: ContactID,
    ) -> Self {
        ContactPoint {
            point,
            normal,
            penetration,
            incident_plane,
            reference_plane,
            id,
            to_incident: Vec2::default(),
            to_reference: Vec2::default(),

            accumulated_normal_impulse: 0.,
            accumulated_tangent_impulse: 0.,
            accumulated_position_bias_impulse: 0.,

            mass_normal: Vec2::default(),
            mass_tangent: Vec2::default(),
            bias: 0.,
        }
    }

    pub fn warm_start(&mut self, other: &Self) {
        self.accumulated_normal_impulse = other.accumulated_normal_impulse;
        self.accumulated_tangent_impulse = other.accumulated_tangent_impulse;
        self.accumulated_position_bias_impulse = other.accumulated_position_bias_impulse;
    }

    pub fn draw(
        &self,
        handle: &mut raylib::prelude::RaylibMode2D<raylib::prelude::RaylibDrawHandle>,
    ) {
        handle.draw_circle(self.point.x as i32, self.point.y as i32, 10., Color::PLUM);

        let end_pos = self.point + self.normal * -self.penetration;
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

    pub fn id(&self) -> &ContactID {
        &self.id
    }
}
