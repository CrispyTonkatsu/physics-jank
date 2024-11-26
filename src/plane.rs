use nalgebra_glm::{vec2, Vec2};

pub struct Plane {
    start: Vec2,
    end: Vec2,
}

impl Plane {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Plane { start, end }
    }

    pub fn get_normal(&self) -> Vec2 {
        // HACK: Probably check if it is pointing outwards
        let to_end = self.end - self.start;
        vec2(to_end.y, -to_end.x)
    }

    pub fn distance_to(&self, point: &Vec2) -> f32 {
        let projected = self.project_point(point);
        let point_to_proj = projected - point;
        let sign = if point_to_proj.dot(&self.get_normal()) >= 0. {
            1.
        } else {
            -1.
        };

        point_to_proj.magnitude() * sign
    }

    pub fn project_point(&self, point: &Vec2) -> Vec2 {
        let start_to_end = self.end - self.start;
        let start_to_point = point - self.start;
        let projected_vec =
            (start_to_end.dot(&start_to_point) / start_to_end.magnitude_squared()) * start_to_end;
        self.start + projected_vec
    }
}
