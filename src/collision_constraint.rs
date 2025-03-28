use std::{cell::RefCell, rc::Rc};

use nalgebra_glm::Vec2;

use crate::{
    body::Body,
    constraints::{Constraint, ContactPoint},
};

pub struct CollisionConstraint {
    manifold: Vec<ContactPoint>,

    incident_body: Rc<RefCell<Body>>,
    reference_body: Rc<RefCell<Body>>,

    old_lambda: f32,
    total_lambda: f32,
}

impl Constraint for CollisionConstraint {
    fn pre_solve(&mut self, inv_dt: f32) {
        let allowed_penetration = 0.01;
        let bias_factor = 0.2;

        for contact in self.manifold.iter_mut() {
            //todo!("Implement the preparing of the math")
        }
    }

    fn solve(&mut self, dt: f32) {
        for contact in self.manifold.iter_mut() {
            //todo!("Implement the applying of impulses")
        }
    }

    fn draw(&self, handle: &mut raylib::prelude::RaylibMode2D<raylib::prelude::RaylibDrawHandle>) {
        for contact in self.manifold.iter() {
            contact.draw(handle);
        }
    }
}

impl CollisionConstraint {
    pub fn new(
        manifold: Vec<ContactPoint>,
        incident_body: Rc<RefCell<Body>>,
        reference_body: Rc<RefCell<Body>>,
    ) -> Self {
        Self {
            manifold,
            incident_body,
            reference_body,
            old_lambda: 0.,
            total_lambda: 0.,
        }
    }

    pub fn update_manifold(&self, manifold: Vec<ContactPoint>) {
        todo!("Implementing the comparison of manifolds such that warm starting takes place")
    }

    pub fn generate_manifold(
        normal: Vec2,
        incident_body: &Body,
        reference_body: &Body,
    ) -> Vec<ContactPoint> {
        let incident_face = incident_body
            .collider_in_world()
            .expect("generate_manifold was somehow called for a body without a polygon.")
            .get_significant_face(-normal);

        let binding = reference_body;
        let reference_polygon = (*binding)
            .collider_in_world()
            .expect("generate_manifold was somehow called for a body without a polygon.");

        let (reference_face, reference_index) =
            reference_polygon.get_significant_face_with_index(normal);

        let next_face =
            reference_polygon.get_plane((reference_index + 1) % reference_polygon.point_count());

        let prev_face = reference_polygon.get_plane(if reference_index == 0 {
            reference_polygon.point_count() - 1
        } else {
            reference_index - 1
        });

        let mut output = vec![incident_face.start(), incident_face.end()];

        if let Some(prev_face_clip) = prev_face.find_intersection(&incident_face) {
            output[1] = prev_face_clip;
        }

        if let Some(next_face_clip) = next_face.find_intersection(&incident_face) {
            output[0] = next_face_clip;
        }

        let (normal, c) = reference_face.get_normal_form();
        output
            .into_iter()
            .filter(|&point| normal.dot(&point) < c)
            .map(|point| {
                ContactPoint::new(
                    point,
                    normal,
                    // NOTE: If for whatever reason the penetration is not using the right sign
                    // check here.
                    reference_face.distance_to(&point),
                    incident_face.clone(),
                    reference_face.clone(),
                )
            })
            .collect()
    }
}
