use crate::geometry::*;
use crate::geometry::{Transform,Matrix};

pub type Index = usize;

#[derive(Default,Debug,Clone)]
pub struct Triangle {
    pub indices: (Index,Index,Index),
    pub p1: Vertex,
    pub p2: Vertex,
    pub p3: Vertex,
}

impl Triangle {

    pub fn normal(&self) -> Normal {
        let p1 = self.p1.clone();
        let p2 = self.p2.clone();
        let p3 = self.p3.clone();

        let a = p2 - p1.clone();
        let b = p3 - p1;

        let x = a.y * b.z - a.z * b.y;
        let y = a.z * b.x - a.x * b.z;
        let z = a.x * b.y - a.y * b.x;

        Normal::new(x,y,z).normalize()
    }
    
    pub fn as_face(self) -> Face {
        Face {
            a: self.indices.0,
            b: self.indices.1,
            c: self.indices.2,
        }
    }

}

impl Transform for Triangle {
    fn transform(&mut self, matrix: &Matrix) {
        self.p1.transform(matrix);
        self.p2.transform(matrix);
        self.p3.transform(matrix);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::geometry::*;

    #[test]
    fn test_triangle_normal() {
        let data = vec![
            Vertex::new(0,0,0),
            Vertex::new(1,0,0),
            Vertex::new(0,1,0),
        ];

        let f = Face::new(1,2,3);
        let t = f.triangle(&data);
        let normal = t.normal();

        assert_eq!(normal.x,0.0);
        assert_eq!(normal.y,0.0);
        assert_eq!(normal.z,1.0);
    }

}