use std::{cell::RefCell, rc::Rc};

use nalgebra_glm::Vec2;
use raylib::prelude::{RaylibDrawHandle, RaylibMode2D};

use crate::{body::Body, collision_info::ContactPoints};

pub trait Constraint {
    /// This function describes how it solves the constraint iteratively.
    fn solve(&mut self, dt: f32);

    fn draw(&self, handle: &mut RaylibMode2D<RaylibDrawHandle>);
}

pub struct CollisionConstraint {
    normal: Vec2,
    penetration: f32,
    manifold: ContactPoints,

    incident_body: Rc<RefCell<Body>>,
    reference_body: Rc<RefCell<Body>>,
}

impl CollisionConstraint {
    pub fn new(
        normal: Vec2,
        penetration: f32,
        manifold: ContactPoints,
        incident_body: Rc<RefCell<Body>>,
        reference_body: Rc<RefCell<Body>>,
    ) -> Self {
        Self {
            normal,
            penetration,
            manifold,
            incident_body,
            reference_body,
        }
    }
}

impl Constraint for CollisionConstraint {
    fn solve(&mut self, _dt: f32) {}

    fn draw(&self, handle: &mut RaylibMode2D<RaylibDrawHandle>) {
        let transform = (*self.reference_body.borrow()).get_transform();
        self.manifold.draw(handle, &transform);
    }
}
