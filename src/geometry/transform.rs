use crate::geometry::Matrix;

pub trait Transform {
    fn transform(&mut self, matrix: &Matrix);
}