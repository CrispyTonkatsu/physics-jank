use std::{cell::RefCell, rc::Rc};

use crate::{body::Body, constraints::CollisionConstraint};
use nalgebra_glm::{Mat3x3, Vec2, Vec3};
use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2D},
};

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
            normal: normal.normalize(),
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

        // BUG: Check that this is working as intended
        // (given it relies on the edges' ordering of counter-clock-wise)
        let mut output = ContactPoints::new(Some(incident_face.start()), Some(incident_face.end()));

        if let Some(prev_face_clip) = prev_face.find_intersection(&incident_face) {
            output.b = Some(prev_face_clip);
        }

        if let Some(next_face_clip) = next_face.find_intersection(&incident_face) {
            output.a = Some(next_face_clip);
        }

        let normal = reference_face.get_normal().normalize();
        output.a = output.a.filter(|&point_a| normal.dot(&point_a) <= 0.);
        output.b = output.b.filter(|&point_b| normal.dot(&point_b) <= 0.);

        output
    }

    pub fn generate_constraint(&self) -> Box<CollisionConstraint> {
        let manifold = self.generate_manifold();

        Box::new(CollisionConstraint::new(
            self.normal,
            self.penetration,
            manifold,
            self.incident_body.clone(),
            self.reference_body.clone(),
        ))
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

    pub fn draw(&self, handle: &mut RaylibMode2D<RaylibDrawHandle>, transform: &Mat3x3) {
        // BUG: Show the manifold using a transform so that it actually goes to the right place

        if let Some(mut point) = self.a {
            point = (transform * Vec3::new(point.x, point.y, 1.)).xy();
            handle.draw_circle(point.x as i32, point.y as i32, 10., Color::RED);
        }

        if let Some(mut point) = self.b {
            point = (transform * Vec3::new(point.x, point.y, 1.)).xy();
            handle.draw_circle(point.x as i32, point.y as i32, 10., Color::RED);
        }
    }
}
