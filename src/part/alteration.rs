use crate::geometry::{
    Vertex,
    Vector,
    Transform,
    MatrixType,
    Matrix
};

/// An Alteration will apply a matrix transformation 
/// of the specified type to a set of points. 
#[derive(Debug,Copy,Clone)]
pub struct Alteration {
    magnitude: f64,        // the multiplier for the change
    dimension: Vector,     // the dimension of the change
    operation: MatrixType, // the type of change to make
}

impl Alteration {

    pub fn new(operation: MatrixType) -> Self {
        Self {
            magnitude: 0.0,
            dimension: Vector::default(),
            operation: operation,
        }
    }

    pub fn with_magnitude(mut self, value: f64) -> Self {
        self.magnitude = value;
        self
    }

    pub fn with_dimension(mut self, value: Vector) -> Self {
        self.dimension = value;
        self
    }

    pub fn build(self) -> Self {
        // verify that magnitude is non-zero
        // verify that at least one dimension is non-zero
        self
    }

    pub fn update(&mut self, value: f64) {
        self.magnitude = value;
    }

    pub fn apply(&self, vertices: &mut Vec<Vertex>) {
        let vector = self.dimension * self.magnitude;

        let matrix = Matrix::matching(
            self.operation,
            vector.x,
            vector.y,
            vector.z,
        );

        for vertex in vertices.iter_mut() {
            vertex.transform(&matrix);
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::models;

    macro_rules! fassert_eq {
        ( $v: expr, $e: expr ) => {
            assert_relative_eq!($v,$e, epsilon = f64::EPSILON);
        }
    }

    #[test]
    fn test_alteration_translate() {
        let change = Alteration::new(MatrixType::Translate)
            .with_dimension(Vector::new(1.0,1.0,1.0))
            .with_magnitude(2.0)
            .build();

        let mut data = vec![
            Vector::new(1.0,1.0,1.0),
            Vector::new(2.0,1.0,1.0),
            Vector::new(3.0,1.0,1.0),
        ];

        change.apply(&mut data);

        fassert_eq!(data[0].x, 3.0);
        fassert_eq!(data[0].y, 3.0);
        fassert_eq!(data[0].z, 3.0);

        fassert_eq!(data[1].x, 4.0);
        fassert_eq!(data[1].y, 3.0);
        fassert_eq!(data[1].z, 3.0);

        fassert_eq!(data[2].x, 5.0);
        fassert_eq!(data[2].y, 3.0);
        fassert_eq!(data[2].z, 3.0);
    }

    #[test]
    fn test_alteration_scaling() {
        let change = Alteration::new(MatrixType::Scale)
            .with_dimension(Vector::new(1.0,1.0,2.0))
            .with_magnitude(2.0)
            .build();

        let mut data = vec![
            Vector::new(1.0,1.0,1.0),
            Vector::new(2.0,1.0,1.0),
            Vector::new(3.0,1.0,1.0),
        ];

        change.apply(&mut data);

        fassert_eq!(data[0].x, 2.0);
        fassert_eq!(data[0].y, 2.0);
        fassert_eq!(data[0].z, 4.0);

        fassert_eq!(data[1].x, 4.0);
        fassert_eq!(data[1].y, 2.0);
        fassert_eq!(data[1].z, 4.0);

        fassert_eq!(data[2].x, 6.0);
        fassert_eq!(data[2].y, 2.0);
        fassert_eq!(data[2].z, 4.0);
    }

    #[test]
    fn test_alteration_rotation() {
        let change = Alteration::new(MatrixType::Rotate)
            .with_dimension(Vector::new(1.0,1.0,1.0))
            .with_magnitude(1.0)
            .build();

        let mut data = vec![
            Vector::new(1.0,2.0,3.0)
        ];

        change.apply(&mut data);

        fassert_eq!(data[0].x,1.9070421093244363);
        fassert_eq!(data[0].y,-1.134517035937589);
        fassert_eq!(data[0].z,3.0126502432958917);
    }

}