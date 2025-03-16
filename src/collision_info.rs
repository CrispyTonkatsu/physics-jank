use std::{cell::RefCell, rc::Rc};

use crate::body::Body;
use nalgebra_glm::Vec2;

#[derive(Debug)]
pub struct CollisionInfo {
    pub normal: Vec2,
    penetration: f32,
    body_a: Rc<RefCell<Body>>,
    body_b: Rc<RefCell<Body>>,
}

impl CollisionInfo {
    pub fn new(
        normal: Vec2,
        penetration: f32,
        body_a: Rc<RefCell<Body>>,
        body_b: Rc<RefCell<Body>>,
    ) -> Self {
        Self {
            normal,
            penetration,
            body_a,
            body_b,
        }
    }
}
