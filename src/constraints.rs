use std::{cell::RefCell, rc::Rc};

use nalgebra_glm::Vec2;
use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2D},
};

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

    old_lambda: f32,
    total_lambda: f32,
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
            old_lambda: 0.,
            total_lambda: 0.,
        }
    }

    fn get_jacobian(&self) -> (Vec2, f32, Vec2, f32) {
        let normal = self.normal;
        let a_to_contact = self.get_body_to_point(&self.incident_body.borrow());
        let b_to_contact = self.get_body_to_point(&self.reference_body.borrow());

        (
            normal,
            ((a_to_contact.x * normal.y) - (a_to_contact.y * normal.x)),
            -normal,
            -((b_to_contact.x * normal.y) - (b_to_contact.y * normal.x)),
        )
    }

    fn get_velocities(&self) -> (Vec2, f32, Vec2, f32) {
        (
            self.incident_body.borrow().velocity(),
            self.incident_body.borrow().angular_velocity(),
            self.reference_body.borrow().velocity(),
            self.reference_body.borrow().angular_velocity(),
        )
    }

    // TODO: Account for the static bodies here
    fn get_inv_mass_row(&self) -> (f32, f32, f32, f32) {
        (
            1. / self.incident_body.borrow().mass(),
            1. / self.incident_body.borrow().inertia(),
            1. / self.reference_body.borrow().mass(),
            1. / self.reference_body.borrow().inertia(),
        )
    }

    fn get_body_to_point(&self, body: &Body) -> Vec2 {
        let point = if let Some(plane) = self.manifold.get_plane() {
            plane.midpoint()
        } else if let Some(point) = self.manifold.get_point() {
            point
        } else {
            panic!("All manifolds should have either 1 or 2 points")
        };

        point - body.center_of_gravity()
    }
}

impl Constraint for CollisionConstraint {
    // This is literally moon runes but also it is the nicest way to display sparse matrix
    // multiplication without making a literal wall of text
    fn solve(&mut self, dt: f32) {
        // HACK: left off here, try to solve both points separately (that could be the issue rn
        // where energy is added) (look at box2d-lite)
        // if that doesnt work then try to ignore the collision if it is already colliding

        let (ja, jb, jc, jd) = self.get_jacobian();
        let (va, ra, vb, rb) = self.get_velocities();
        let jv = (ja.dot(&va)) + (jb * ra) + (jc.dot(&vb)) + (jd * rb);

        let (ma, ia, mb, ib) = self.get_inv_mass_row();
        let effective_mass = 1. / (ma + (jb * jb * ia) + mb + (jd * jd * ib));

        // TODO: Insert b here where it will account for the coefficient of restitution
        let b = -(0.01 / dt) * self.penetration;

        let lambda = effective_mass * (-jv + b);

        self.old_lambda = self.total_lambda;
        self.total_lambda = f32::clamp(self.total_lambda + lambda, 0., f32::MAX);

        let lambda = self.total_lambda - self.old_lambda;

        let mut body_a = self.incident_body.borrow_mut();
        body_a.apply_impulse(ma * ja * lambda);
        body_a.apply_angular_impulse(ia * jb * lambda);

        let mut body_b = self.reference_body.borrow_mut();
        body_b.apply_impulse(mb * jc * lambda);
        body_b.apply_angular_impulse(ib * jd * lambda);

        // TODO: Insert the applying of the friction here
    }

    fn draw(&self, handle: &mut RaylibMode2D<RaylibDrawHandle>) {
        self.manifold.draw(handle);

        (*self.incident_body.borrow_mut())
            .collider_in_world()
            .expect("Generate manifold was somehow called for a body without a polygon.")
            .get_significant_face(-self.normal)
            .draw(handle, &Color::RED);

        (*self.reference_body.borrow_mut())
            .collider_in_world()
            .expect("Generate manifold was somehow called for a body without a polygon.")
            .get_significant_face(self.normal)
            .draw(handle, &Color::BLUE);

        let c_of_g = self.reference_body.borrow().center_of_gravity();
        let start_pos = Vector2::new(c_of_g.x, c_of_g.y);
        let end_pos = start_pos + Vector2::new(self.normal.x, self.normal.y) * 100.;
        handle.draw_line_ex(start_pos, end_pos, 2., Color::GREEN);
    }
}
