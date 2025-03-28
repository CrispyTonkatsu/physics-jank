use raylib::prelude::{RaylibDrawHandle, RaylibMode2D};

pub trait Constraint {
    fn pre_solve(&mut self, dt: f32);

    fn solve(&mut self, dt: f32);

    fn draw(&self, handle: &mut RaylibMode2D<RaylibDrawHandle>);
}
