use std::convert::TryFrom;

use crate::utilities;
use crate::geometry::*;
use crate::errors::Error;
use crate::constant::{FACE_TAG,Index};

#[derive(Default,Debug,Clone)]
pub struct Face {
    pub a: Index,
    pub b: Index,
    pub c: Index,
}

impl Face {

    // Assumes that values given are 1-indexed
    pub const fn new(a: usize, b: usize, c: usize) -> Self {
        Self { 
            a: a.saturating_sub(1),
            b: b.saturating_sub(1),
            c: c.saturating_sub(1),
        }
    }

    // Assumes that values given are 1-indexed
    pub fn with<T: Into<usize>>((a,b,c): (T,T,T)) -> Self {
        Self::make(a,b,c)
    }

    // Assumes that values given are 1-indexed
    pub fn make<T: Into<usize>>(a: T, b: T, c: T) -> Self {
        Self {
            a: a.into().saturating_sub(1),
            b: b.into().saturating_sub(1),
            c: c.into().saturating_sub(1),
        }
    }

    pub fn is_valid(&self, data: &Vec<Vertex>) -> bool {
        let l = data.len();
        self.a < l && 
        self.b < l &&
        self.c < l
    }

    pub fn normal(&self, data: &Vec<Vertex>) -> Normal {
        self.triangle(data).normal()
    }

    pub fn triangle(&self, data: &Vec<Vertex>) -> Triangle {
        let p1 = data[self.a].clone();
        let p2 = data[self.b].clone();
        let p3 = data[self.c].clone();
        let indices = (self.a,self.b,self.c);

        Triangle {
            indices,
            p1, 
            p2, 
            p3
        }
    }

}

impl TryFrom<&str> for Face {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Face::from(utilities::extract::<usize>(FACE_TAG,value)?))
    }
}

impl TryFrom<String> for Face {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Face::try_from(value.as_str())
    }
}

impl From<(usize,usize,usize)> for Face {
    fn from(v: (usize,usize,usize)) -> Self {
        Self::with(v)
    }
}

impl From<Face> for String {
    fn from(v: Face) -> Self {
        let a = v.a.saturating_add(1);
        let b = v.b.saturating_add(1);
        let c = v.c.saturating_add(1);
        format!("{} {} {} {}",
            FACE_TAG, a, b, c
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // TODO: add negative numbers check

    #[test]
    fn test_face_from_string() {

        let d = "f 1 3 9".to_string();
        let t  = Face::try_from(d).unwrap();

        assert_eq!(t.a,0);
        assert_eq!(t.b,2);
        assert_eq!(t.c,8);
    }

    #[test]
    fn test_string_from_face() {
        let t = Face::new(1,3,9);

        let d1 = "f 1 3 9".to_string();
        let d2 = String::from(t);

        assert_eq!(d1,d2);
    }

    #[test]
    fn test_face_normal() {
        let data = vec![
            Vertex::new(0.0,0.0,0.0),
            Vertex::new(1.0,0.0,0.0),
            Vertex::new(0.0,1.0,0.0),
        ];

        let t = Face::new(1,2,3);
        let normal = t.normal(&data);

        assert_eq!(normal.x,0.0);
        assert_eq!(normal.y,0.0);
        assert_eq!(normal.z,1.0);
    }

}