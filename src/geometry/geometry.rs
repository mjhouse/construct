use std::convert::TryFrom;
use itertools::Itertools;

use crate::errors::Error;
use crate::geometry::*;

#[derive(Default,Debug,Clone)]
pub struct Geometry {
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
}

impl Geometry {

    pub fn make(values: Vec<f64>, indices: Vec<usize>) -> Self {

        let vertices = values
            .as_slice()
            .chunks_exact(3)
            .map(|k| Vertex::new(k[0],k[1],k[2]) )
            .collect();

        let faces = indices
            .as_slice()
            .chunks_exact(3)
            .map(|k| Face::new(k[0],k[1],k[2]) )
            .collect();

        Self::new(vertices,faces)
    }

    pub const fn new(vertices: Vec<Vertex>, faces: Vec<Face>) -> Self {
        Self { vertices, faces }
    }

    pub fn size(&self) -> usize {
        self.faces.len()
    }

    pub fn get(&self, i: usize) -> Triangle {
        let face = &self.faces[i];
        face.triangle(&self.vertices)
    }

    pub fn validated(self) -> Result<Self,Error> {
        for face in self.faces.iter() {
            if !face.is_valid(&self.vertices) {
                return Err(Error::ParseError);
            }
        }
        Ok(self)
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn vertices_mut(&mut self) -> &mut Vec<Vertex> {
        &mut self.vertices
    }

}

impl IntoIterator for Geometry {
    type Item = Triangle;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.faces
            .into_iter()
            .map(|f| f.triangle(&self.vertices))
            .collect::<Vec<Triangle>>()
            .into_iter()
    }
}

impl TryFrom<String> for Geometry {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut geometry = Geometry::default();

        for line in value.lines() {
            if let Ok(v) = Vertex::try_from(line) {
                geometry.vertices.push(v);
                continue;
            }
            if let Ok(f) = Face::try_from(line) {
                geometry.faces.push(f);
                continue;
            }
        }

        geometry.validated()
    }
}

impl From<Geometry> for String {
    fn from(geometry: Geometry) -> Self {
        let mut result = String::new();

        let vertices = Itertools::intersperse(
            geometry.vertices
                .into_iter()
                .map(String::from),
            "\n".into()
        ).collect::<String>();

        let faces = Itertools::intersperse(
            geometry.faces
                .into_iter()
                .map(String::from),
            "\n".into()
        ).collect::<String>();

        result.push_str(&vertices);
        result.push_str("\n");
        result.push_str(&faces);
        result
    }
}

impl Transform for Geometry {
    fn transform(&mut self, matrix: &Matrix) {
        self.vertices.transform(matrix);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_string_from_geometry() {
        let d = "\
            v 0.1 0.2 0.3\n\
            v 0.4 0.5 0.6\n\
            v 0.7 0.8 0.9\n\
            v 1.1 1.2 1.3\n\
            v 1.4 1.5 1.6\n\
            v 1.7 1.8 1.9\n\
            f 1 2 3\n\
            f 4 5 6\n"
        .to_string();

        let g = Geometry::try_from(d.clone()).unwrap();
        let s = String::from(g.clone());

        assert_eq!(g.size(),2);
        assert_eq!(d.trim(),s.trim());
    }

    #[test]
    fn test_geometry_from_string() {
        let d = "\
            v 0.1 0.2 0.3\n\
            v 0.4 0.5 0.6\n\
            v 0.7 0.8 0.9\n\
            v 1.1 1.2 1.3\n\
            v 1.4 1.5 1.6\n\
            v 1.7 1.8 1.9\n\
            f 1 2 3\n\
            f 4 5 6\n"
        .to_string();

        let g = Geometry::try_from(d).unwrap();

        assert_eq!(g.size(),2);

        let a = g.get(0);
        let b = g.get(1);

        assert_eq!(a.p1.x,0.1);
        assert_eq!(a.p1.y,0.2);
        assert_eq!(a.p1.z,0.3);
        assert_eq!(a.p2.x,0.4);
        assert_eq!(a.p2.y,0.5);
        assert_eq!(a.p2.z,0.6);
        assert_eq!(a.p3.x,0.7);
        assert_eq!(a.p3.y,0.8);
        assert_eq!(a.p3.z,0.9);
        assert_eq!(b.p1.x,1.1);
        assert_eq!(b.p1.y,1.2);
        assert_eq!(b.p1.z,1.3);
        assert_eq!(b.p2.x,1.4);
        assert_eq!(b.p2.y,1.5);
        assert_eq!(b.p2.z,1.6);
        assert_eq!(b.p3.x,1.7);
        assert_eq!(b.p3.y,1.8);
        assert_eq!(b.p3.z,1.9);
    }

}