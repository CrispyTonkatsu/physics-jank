use std::{cell::RefCell, rc::Rc};

use nalgebra_glm::Vec2;

use crate::{
    body::Body,
    constraints::Constraint,
    contact_point::{ContactID, ContactPoint},
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

    pub fn update_manifold(&mut self, manifold: Vec<ContactPoint>) {
        self.manifold = manifold
            .into_iter()
            .map(|mut new_contact| {
                for old_contact in self.manifold.iter() {
                    if old_contact.id() == new_contact.id() {
                        new_contact.warm_start(old_contact);
                        break;
                    }
                }

                new_contact
            })
            .collect();
    }

    pub fn generate_manifold(
        normal: Vec2,
        incident_body: &Body,
        reference_body: &Body,
    ) -> Vec<ContactPoint> {
        let incident_polygon = (*incident_body)
            .collider_in_world()
            .expect("generate_manifold was somehow called for a body without a polygon.");

        let (incident_face, incident_index) =
            incident_polygon.get_significant_face_with_index(-normal);

        let reference_polygon = (*reference_body)
            .collider_in_world()
            .expect("generate_manifold was somehow called for a body without a polygon.");

        let (reference_face, reference_index) =
            reference_polygon.get_significant_face_with_index(normal);

        let next_face_index = reference_polygon.increment_side(reference_index);
        let next_face = reference_polygon.get_plane(next_face_index);

        let prev_face_index = reference_polygon.decrement_side(reference_index);
        let prev_face = reference_polygon.get_plane(prev_face_index);

        let mut output = vec![incident_face.start(), incident_face.end()];
        let mut output_id = [
            ContactID::new(
                false,
                incident_index,
                false,
                incident_polygon.decrement_side(incident_index),
            ),
            ContactID::new(
                false,
                incident_index,
                false,
                incident_polygon.increment_side(incident_index),
            ),
        ];

        if let Some(prev_face_clip) = prev_face.find_intersection(&incident_face) {
            output[1] = prev_face_clip;
            output_id[1].set_second_edge(true, prev_face_index);
        }

        if let Some(next_face_clip) = next_face.find_intersection(&incident_face) {
            output[0] = next_face_clip;
            output_id[0].set_second_edge(true, next_face_index);
        }

        let (normal, c) = reference_face.get_normal_form();
        output
            .into_iter()
            .filter(|&point| normal.dot(&point) < c)
            .enumerate()
            .map(|(i, point)| {
                ContactPoint::new(
                    point,
                    normal,
                    reference_face.distance_to(&point),
                    incident_face.clone(),
                    reference_face.clone(),
                    output_id[i].clone(),
                )
            })
            .collect()
    }
}
