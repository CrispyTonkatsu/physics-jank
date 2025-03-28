use catppuccin::ColorName;
use nalgebra_glm::Vec2;
use raylib::prelude::*;

use crate::{body::Body, color, plane::Plane};

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

    effective_mass: f32,
    tangent_mass: f32,

    bias: f32,

    id: ContactID,

    incident_plane: Plane,
    reference_plane: Plane,
}

impl ContactPoint {
    pub fn warm_start(&mut self, other: &Self) {
        self.accumulated_normal_impulse = other.accumulated_normal_impulse;
        self.accumulated_tangent_impulse = other.accumulated_tangent_impulse;
        self.accumulated_position_bias_impulse = other.accumulated_position_bias_impulse;
    }

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
            id,
            to_incident: Vec2::default(),
            to_reference: Vec2::default(),

            accumulated_normal_impulse: 0.,
            accumulated_tangent_impulse: 0.,
            accumulated_position_bias_impulse: 0.,

            effective_mass: f32::default(),
            tangent_mass: f32::default(),
            bias: 0.,

            incident_plane,
            reference_plane,
        }
    }

    pub fn apply_accumulated_impulses(
        &mut self,
        incident_body: &mut Body,
        reference_body: &mut Body,
    ) {
        let tangent = Vec2::new(-self.normal().y, self.normal().x);
        let impulse = self.accumulated_normal_impulse() * self.normal()
            + self.accumulated_tangent_impulse() * tangent;

        let cross = |a: Vec2, b: Vec2| -> f32 { a.x * b.y - a.y * b.x };

        incident_body.apply_impulse(impulse);
        incident_body.apply_angular_impulse(cross(self.to_incident(), impulse));

        reference_body.apply_impulse(-impulse);
        reference_body.apply_angular_impulse(cross(self.to_reference(), -impulse));
    }

    pub fn draw(
        &self,
        handle: &mut raylib::prelude::RaylibMode2D<raylib::prelude::RaylibDrawHandle>,
    ) {
        self.incident_plane
            .draw(handle, &color::get(ColorName::Blue));
        self.reference_plane
            .draw(handle, &color::get(ColorName::Yellow));

        handle.draw_circle(
            self.point.x as i32,
            self.point.y as i32,
            10.,
            color::get(ColorName::Green),
        );

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
            color::get(ColorName::Peach),
        );
    }

    pub fn id(&self) -> &ContactID {
        &self.id
    }

    pub fn point(&self) -> Vec2 {
        self.point
    }

    pub fn normal(&self) -> Vec2 {
        self.normal
    }

    pub fn penetration(&self) -> f32 {
        self.penetration
    }

    pub fn to_incident(&self) -> Vec2 {
        self.to_incident
    }

    pub fn set_to_incident(&mut self, to_incident: Vec2) {
        self.to_incident = to_incident;
    }

    pub fn to_reference(&self) -> Vec2 {
        self.to_reference
    }

    pub fn set_to_reference(&mut self, to_reference: Vec2) {
        self.to_reference = to_reference;
    }

    pub fn accumulated_normal_impulse(&self) -> f32 {
        self.accumulated_normal_impulse
    }

    pub fn set_accumulated_normal_impulse(&mut self, accumulated_normal_impulse: f32) {
        self.accumulated_normal_impulse = accumulated_normal_impulse;
    }

    pub fn accumulated_tangent_impulse(&self) -> f32 {
        self.accumulated_tangent_impulse
    }

    pub fn set_accumulated_tangent_impulse(&mut self, accumulated_tangent_impulse: f32) {
        self.accumulated_tangent_impulse = accumulated_tangent_impulse;
    }

    pub fn accumulated_position_bias_impulse(&self) -> f32 {
        self.accumulated_position_bias_impulse
    }

    pub fn set_accumulated_position_bias_impulse(
        &mut self,
        accumulated_position_bias_impulse: f32,
    ) {
        self.accumulated_position_bias_impulse = accumulated_position_bias_impulse;
    }

    pub fn effective_mass(&self) -> f32 {
        self.effective_mass
    }

    pub fn set_tangent_mass(&mut self, tangent_mass: f32) {
        self.tangent_mass = tangent_mass;
    }

    pub fn set_effective_mass(&mut self, effective_mass: f32) {
        self.effective_mass = effective_mass;
    }

    pub fn tangent_mass(&self) -> f32 {
        self.tangent_mass
    }

    pub fn bias(&self) -> f32 {
        self.bias
    }

    pub fn set_bias(&mut self, bias: f32) {
        self.bias = bias;
    }
}
