use crate::Value;

pub trait Plotter {
    fn plot(&self, points: Vec<(f32, f32)>) -> Value;
}
