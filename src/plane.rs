use nalgebra_glm::{vec2, Vec2};
use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2D},
};

pub struct Plane {
    start: Vec2,
    end: Vec2,
}

impl Plane {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Plane { start, end }
    }

    pub fn get_normal(&self) -> Vec2 {
        let to_end = self.end - self.start;
        vec2(to_end.y, -to_end.x)
    }

    pub fn distance_to(&self, point: &Vec2) -> f32 {
        let projected = self.project_point(point);
        let proj_to_point = point - projected;

        let sign = if proj_to_point.dot(&self.get_normal()) >= 0. {
            1.
        } else {
            -1.
        };

        proj_to_point.magnitude() * sign
    }

    pub fn project_point(&self, point: &Vec2) -> Vec2 {
        let start_to_end = self.end - self.start;
        let start_to_point = point - self.start;
        let projected_vec =
            (start_to_end.dot(&start_to_point) / start_to_end.magnitude_squared()) * start_to_end;
        self.start + projected_vec
    }

    pub fn find_intersection(&self, other: &Plane) -> Option<Vec2> {
        let (normal, c) = self.get_normal_form();
        let (start, v) = other.get_parametric_form();

        let t = (c - start.dot(&normal)) / (v.dot(&normal));

        if (0. ..=1.).contains(&t) {
            Some(start + (t * v))
        } else {
            None
        }
    }

    pub fn get_parametric_form(&self) -> (Vec2, Vec2) {
        (self.start, self.end - self.start)
    }

    pub fn get_normal_form(&self) -> (Vec2, f32) {
        let normal = self.get_normal().normalize();

        (normal, normal.dot(&self.start))
    }

    pub fn is_made_of(&self, point: &Vec2) -> bool {
        point.eq(&self.start) || point.eq(&self.end)
    }

    pub fn start(&self) -> Vec2 {
        self.start
    }

    pub fn end(&self) -> Vec2 {
        self.end
    }

    pub fn midpoint(&self) -> Vec2 {
        (self.start() + self.end()) * (1. / 2.)
    }

    pub fn draw(&self, handle: &mut RaylibMode2D<RaylibDrawHandle>, color: &Color) {
        handle.draw_line_ex(
            Vector2::new(self.start.x, self.start.y),
            Vector2::new(self.end.x, self.end.y),
            5.,
            color,
        );
    }
}
