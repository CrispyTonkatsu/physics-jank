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
}

impl Constraint for CollisionConstraint {
    // HACK: Left off here, going over the math to find the mistake related to rotations
    fn pre_solve(&mut self, inv_dt: f32) {
        let allowed_penetration = 0.01;
        let bias_factor = 0.2;

        let mut incident_body = self.incident_body.borrow_mut();
        let mut reference_body = self.reference_body.borrow_mut();

        for contact in self.manifold.iter_mut() {
            contact.set_to_incident(contact.point() - incident_body.center_of_gravity());
            contact.set_to_reference(contact.point() - reference_body.center_of_gravity());

            let net_inv_mass = incident_body.inv_mass() + reference_body.inv_mass();

            // Effective mass (mass that affects the linear push done to the contact point)
            let incident_normal_mass = contact.to_incident().dot(&contact.normal());
            let reference_normal_mass = contact.to_reference().dot(&contact.normal());

            let net_normal_mass = net_inv_mass
                + incident_body.inv_inertia()
                    * (contact.to_incident().norm_squared()
                        - (incident_normal_mass * incident_normal_mass))
                + reference_body.inv_inertia()
                    * (contact.to_reference().norm_squared()
                        - (reference_normal_mass * reference_normal_mass));

            contact.set_effective_mass(1. / net_normal_mass);

            // Tangent mass
            let tangent = Vec2::new(-contact.normal().y, contact.normal().x);
            let incident_tangent_mass = contact.to_incident().dot(&tangent);
            let reference_tangent_mass = contact.to_reference().dot(&tangent);

            let net_tangent_mass = net_inv_mass
                + incident_body.inv_inertia()
                    * (contact.to_incident().norm_squared()
                        - (incident_tangent_mass * incident_tangent_mass))
                + reference_body.inv_inertia()
                    * (contact.to_reference().norm_squared()
                        - (reference_tangent_mass * reference_tangent_mass));

            contact.set_tangent_mass(1. / net_tangent_mass);

            // Setting the bias
            contact.set_bias(
                -bias_factor * inv_dt * (contact.penetration() + allowed_penetration).min(0.),
            );

            // Applying accumulated impulses
            contact.apply_accumulated_impulses(&mut incident_body, &mut reference_body);
        }
    }

    fn solve(&mut self) {
        let mut incident_body = self.incident_body.borrow_mut();
        let mut reference_body = self.reference_body.borrow_mut();

        let angular_to_tangent = |a: f32, b: Vec2| -> Vec2 { Vec2::new(-a * b.y, a * b.x) };

        let cross = |a: Vec2, b: Vec2| -> f32 { a.x * b.y - a.y * b.x };

        for contact in self.manifold.iter_mut() {
            let relative_velocity = incident_body.velocity()
                + angular_to_tangent(incident_body.angular_velocity(), contact.to_incident())
                - reference_body.velocity()
                - angular_to_tangent(reference_body.angular_velocity(), contact.to_reference());

            let normal_impulse = contact.effective_mass()
                * (-relative_velocity.dot(&contact.normal()) + contact.bias());

            // Clamping
            let to_apply = (contact.accumulated_normal_impulse() + normal_impulse).max(0.);
            contact.set_accumulated_normal_impulse(to_apply);

            let to_apply = (contact.accumulated_normal_impulse() - to_apply) * contact.normal();

            // Applying the normal impulse

            incident_body.apply_impulse(to_apply);
            incident_body.apply_angular_impulse(cross(contact.to_incident(), to_apply));

            reference_body.apply_impulse(-to_apply);
            reference_body.apply_angular_impulse(cross(contact.to_reference(), -to_apply));
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
