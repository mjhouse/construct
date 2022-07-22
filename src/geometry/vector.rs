use std::fmt;
use std::ops::Sub;
use std::convert::TryFrom;

use crate::utilities;
use crate::geometry::{Transform,Matrix};
use crate::errors::Error;
use crate::constant::VERTEX_TAG;

#[derive(Default,Debug,Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Vertex = Vector;
pub type Normal = Vertex;

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Vector {

    pub fn with<T: Into<f64>>((x,y,z): (T,T,T)) -> Self {
        Self::new(x,y,z)
    }
    
    pub fn new<T: Into<f64>>(x: T, y: T, z: T) -> Self {
        Self { 
            x: x.into(), 
            y: y.into(), 
            z: z.into()
        }
    }

    pub fn magnitude(&self) -> f64 {
        let v1 = self.x * self.x;
        let v2 = self.y * self.y;
        let v3 = self.z * self.z;
        (v1 + v2 + v3).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let mut v = self.clone();
        let m = v.magnitude();
        if m > 0.0 {
            v.x /= m;
            v.y /= m;
            v.z /= m;
        }
        v
    }

    pub fn unpack(&self) -> (f64,f64,f64) {
        (self.x,self.y,self.z)
    }

}

impl TryFrom<&str> for Vector {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Vertex::from(utilities::extract::<f64>(VERTEX_TAG,value)?))
    }
}

impl TryFrom<String> for Vector {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Vector::try_from(value.as_str())
    }
}

impl<T> From<(T,T,T)> for Vector 
where
    T: Into<f64>
{
    fn from(v: (T,T,T)) -> Self {
        Self::with(v)
    }
}

impl From<&Vector> for String {
    fn from(v: &Vector) -> Self {
        format!("{} {} {} {}",
            VERTEX_TAG, v.x, v.y, v.z
        )
    }
}

impl From<Vector> for String {
    fn from(v: Vector) -> Self {
        (&v).into()
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Transform for Vector {
    fn transform(&mut self, matrix: &Matrix) {
        let [
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44
        ] = matrix.unpack();

        let (x,y,z) = self.unpack();
        let w = 1.0;

        let dw = m41 * x + m42 * y + m43 * z + m44 * w;
        let dx = m11 * x + m12 * y + m13 * z + m14 * w;
        let dy = m21 * x + m22 * y + m23 * z + m24 * w;
        let dz = m31 * x + m32 * y + m33 * z + m34 * w;

        self.x = dx / dw;
        self.y = dy / dw;
        self.z = dz / dw;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_string_from_vector_int() {
        let data = "v 1 5 9".to_string();
        let vector = Vector::new(1,5,9);
        assert_eq!(String::from(vector),data);
    }

    #[test]
    fn test_string_from_vector() {
        let data = "v 1.1234 5.4321 9.87642343".to_string();
        let vector = Vector::new(1.1234,5.4321,9.87642343);
        assert_eq!(String::from(vector),data);
    }

    #[test]
    fn test_vector_from_string() {
        let data = "v 1.1234 5.4321 9.87642343".to_string();
        let vector = Vector::try_from(data).unwrap();

        // capture the first 4 trailing digits
        let k = 1000.0;
        let v0 = (vector.x * k).round();
        let v1 = (vector.y * k).round();
        let v2 = (vector.z * k).round();

        assert_eq!(v0,1123.0);
        assert_eq!(v1,5432.0);
        assert_eq!(v2,9876.0);
    }

    #[test]
    fn test_vector_from_string_int() {
        let data = "v 1 5 9".to_string();
        let vector = Vector::try_from(data).unwrap();

        assert_eq!(vector.x,1.0);
        assert_eq!(vector.y,5.0);
        assert_eq!(vector.z,9.0);
    }

    #[test]
    fn test_vector_magnitude() {
        let vector = Vector::new(1,2,2);
        let result = vector.magnitude();
        assert_eq!(result as u64,3);
    }

    #[test]
    fn test_vector_normalize() {
        let vector = Vector::new(1,2,2);
        let result = vector.normalize();

        // capture the first 4 trailing digits
        let k = 10000.0;
        let v0 = (result.x * k).round();
        let v1 = (result.y * k).round();
        let v2 = (result.z * k).round();

        // verify that they are expected values
        assert_eq!(v0,3333.0);
        assert_eq!(v1,6667.0);
        assert_eq!(v2,6667.0);
    }

    #[test]
    fn test_vector_sub() {
        let vector1 = Vector::new(2,2,2);
        let vector2 = Vector::new(1,1,1);
        let vector3 = vector1 - vector2;

        assert_eq!(vector3.x,1.0);
        assert_eq!(vector3.y,1.0);
        assert_eq!(vector3.z,1.0);
    }

}
