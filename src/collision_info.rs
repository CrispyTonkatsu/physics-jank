use std::{cell::RefCell, rc::Rc};

use crate::{body::Body, constraints::CollisionConstraint};
use nalgebra_glm::Vec2;

#[derive(Debug)]
pub struct CollisionInfo {
    pub normal: Vec2,
    pub penetration: f32,
    pub incident_body: Rc<RefCell<Body>>,
    pub reference_body: Rc<RefCell<Body>>,
}

impl CollisionInfo {
    pub fn new(
        normal: Vec2,
        penetration: f32,
        incident_body: Rc<RefCell<Body>>,
        reference_body: Rc<RefCell<Body>>,
    ) -> Self {
        Self {
            normal,
            penetration,
            incident_body,
            reference_body,
        }
    }

    fn generate_manifold(&self) -> ContactPoints {
        // HACK: If the normal was used with the wrong sign, try flipping this in case it doesn't
        // work

        let incident_face = (*self.incident_body.borrow_mut())
            .collider()
            .expect("Generate manifold was somehow called for a body without a polygon.")
            .get_significant_face(-self.normal);

        let binding = self.reference_body.borrow();
        let reference_polygon = (*binding)
            .collider()
            .expect("Generate manifold was somehow called for a body without a polygon.");

        let (reference_face, reference_index) =
            reference_polygon.get_significant_face_with_index(self.normal);

        let next_face =
            reference_polygon.get_plane((reference_index + 1) % reference_polygon.point_count());

        let prev_face = reference_polygon.get_plane(
            if reference_index == 0 {
                reference_polygon.point_count()
            } else {
                reference_index
            } - 1,
        );

        // BUG: Check that this is working as intended (given it relies on the edges mapping very
        // specifically)
        let mut output = ContactPoints::new(Some(incident_face.start()), Some(incident_face.end()));

        if let Some(prev_face_clip) = prev_face.find_intersection(&incident_face) {
            output.a = Some(prev_face_clip);
        }

        if let Some(next_face_clip) = next_face.find_intersection(&incident_face) {
            output.b = Some(next_face_clip);
        }

        let normal = reference_face.get_normal().normalize();
        output.a = output.a.filter(|&point_a| normal.dot(&point_a) <= 0.);
        output.b = output.b.filter(|&point_b| normal.dot(&point_b) <= 0.);

        output
    }

    pub fn generate_constraint(&self) -> Box<CollisionConstraint> {
        let _manifold = self.generate_manifold();
        todo!("Can't generate constraints yet")
    }
}

/// This describes how the polygons are overlapping with each other. This could be more ergonomic
/// but there is more to build up to.
pub struct ContactPoints {
    pub a: Option<Vec2>,
    pub b: Option<Vec2>,
}

impl ContactPoints {
    pub fn new(a: Option<Vec2>, b: Option<Vec2>) -> Self {
        Self { a, b }
    }

    pub fn is_valid(&self) -> bool {
        !(self.a.is_none() && self.b.is_none())
    }

    /// Returns the edge if there are 2 points in this manifold
    pub fn get_edge(&self) -> Option<(Vec2, Vec2)> {
        self.a.zip(self.b)
    }

    /// Returns the point if there is only one
    pub fn get_point(&self) -> Option<Vec2> {
        if self.a.is_some() && self.b.is_some() {
            return None;
        }

        self.a.or(self.b)
    }
}
