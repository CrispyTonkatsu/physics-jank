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
        vec2(-to_end.y, to_end.x)
    }

    pub fn distance_to(&self, point: &Vec2) -> f32 {
        // TODO: Left off here. Calculate distance to this point (make sure its done correctly)
        0.
    }
}
