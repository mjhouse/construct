use std::iter::zip;
use log;

use crate::geometry::{Matrix,Vector,Vertex,Transform,Geometry};
use crate::constant::Index;
use crate::errors::Error;
use crate::part::Alteration;

#[derive(Debug,Clone)]
pub enum Selection {
    Specific(Vec<Index>),
    Range((Index,Index)),
    All,
}

#[derive(Debug,Clone)]
pub struct AttributeItem {
    selection:  Selection,
    alteration: Alteration,
}

#[derive(Debug,Clone)]
pub struct Attribute {
    name:  String,
    items: Vec<AttributeItem>,
}

impl Selection {

    pub fn specific<T: Into<Vec<Index>>>(indices: T) -> Self {
        Self::Specific(indices.into())
    }

    pub fn range(start: usize, end: usize) -> Self {
        Self::Range((start,end))
    }

    pub fn all() -> Self {
        Self::All
    }

    pub fn apply(&self, alteration: &Alteration, vertices: &mut Vec<Vertex>) {
        match self.clone() {
            Selection::Specific(v) => self.apply_specific(v,alteration,vertices),
            Selection::Range(v) => self.apply_range(v,alteration,vertices),
            Selection::All => self.apply_all(alteration,vertices)
        }
    }

    fn apply_specific(&self, indices: Vec<Index>, alteration: &Alteration, vertices: &mut Vec<Vertex>) {
        let matrix = alteration.matrix();
        for index in indices.into_iter() {
            vertices[index].transform(&matrix);
        }
    }

    fn apply_range(&self, (start,end): (Index,Index), alteration: &Alteration, vertices: &mut Vec<Vertex>) {
        let matrix = alteration.matrix();
        for vertex in vertices[start..end].iter_mut() {
            vertex.transform(&matrix);
        }
    }

    fn apply_all(&self, alteration: &Alteration, vertices: &mut Vec<Vertex>) {
        let matrix = alteration.matrix();
        for vertex in vertices.iter_mut() {
            vertex.transform(&matrix);
        }
    }

    pub fn centroid(&self, vertices: &Vec<Vertex>) -> Vertex {
        match self.clone() {
            Selection::Specific(v) => self.centroid_specific(v,vertices),
            Selection::Range(v) => self.centroid_range(v,vertices),
            Selection::All => self.centroid_all(vertices)
        }
    }

    fn centroid_specific(&self, indices: Vec<Index>, vertices: &Vec<Vertex>) -> Vertex {
        let mut result = Vertex::new(0.0,0.0,0.0);
        let length = indices.len();

        for index in indices.into_iter() {
            result.x += vertices[index].x;
            result.y += vertices[index].y;
            result.z += vertices[index].z;
        }

        result / length
    }

    fn centroid_range(&self, (start,end): (Index,Index), vertices: &Vec<Vertex>) -> Vertex {
        let mut result = Vertex::new(0.0,0.0,0.0);
        let mut count = 0;

        for vertex in vertices[start..end].iter() {
            result.x += vertex.x;
            result.y += vertex.y;
            result.z += vertex.z;
            count += 1;
        }

        result / count
    }

    fn centroid_all(&self, vertices: &Vec<Vertex>) -> Vertex {
        let mut result = Vertex::new(0.0,0.0,0.0);
        let mut count = 0;

        for vertex in vertices.iter() {
            result.x += vertex.x;
            result.y += vertex.y;
            result.z += vertex.z;
            count += 1;
        }

        result / count
    }

}

impl AttributeItem {

    fn new(selection: Selection, alteration: Alteration) -> Self {
        Self { selection, alteration }
    }

    pub fn scale_specific<T: Into<Vec<Index>>>(dimension: Vector, indices: T) -> Self {
        Self::new(
            Selection::specific(indices),
            Alteration::scale(dimension)
        )
    }

    pub fn scale_range(dimension: Vector, start: usize, end: usize) -> Self {
        Self::new(
            Selection::range(start,end),
            Alteration::scale(dimension)
        )
    }

    pub fn scale_all(dimension: Vector) -> Self {
        Self::new(
            Selection::all(),
            Alteration::scale(dimension)
        )
    }

    pub fn rotate_specific<T: Into<Vec<Index>>>(dimension: Vector, indices: T) -> Self {
        Self::new(
            Selection::specific(indices),
            Alteration::rotate(dimension)
        )
    }

    pub fn rotate_range(dimension: Vector, start: usize, end: usize) -> Self {
        Self::new(
            Selection::range(start,end),
            Alteration::rotate(dimension)
        )
    }

    pub fn rotate_all(dimension: Vector) -> Self {
        Self::new(
            Selection::all(),
            Alteration::rotate(dimension)
        )
    }

    pub fn translate_specific<T: Into<Vec<Index>>>(dimension: Vector, indices: T) -> Self {
        Self::new(
            Selection::specific(indices),
            Alteration::translate(dimension)
        )
    }

    pub fn translate_range(dimension: Vector, start: usize, end: usize) -> Self {
        Self::new(
            Selection::range(start,end),
            Alteration::translate(dimension)
        )
    }

    pub fn translate_all(dimension: Vector) -> Self {
        Self::new(
            Selection::all(),
            Alteration::translate(dimension)
        )
    }

    pub fn update_magnitude(&mut self, magnitude: f64) {
        self.alteration.update_magnitude(magnitude);
    }

    pub fn update_dimension(&mut self, dimension: Vector) {
        self.alteration.update_dimension(dimension);
    }

    pub fn apply(&self, vertices: &mut Vec<Vertex>) {
        self.selection.apply(&self.alteration,vertices);
    }

    pub fn centroid(&self, geometry: &Geometry) -> Vertex {
        self.selection.centroid(geometry.vertices())
    }

}

impl Attribute {
    
    pub fn new(name: String, items: Vec<AttributeItem>) -> Self {
        Self { name, items }
    }

    pub fn update(&mut self, value: f64) {
        for item in self.items.iter_mut() {
            item.update_magnitude(value);
        }
    }

    pub fn apply(&self, vertices: &mut Vec<Vertex>) {
        for item in self.items.iter() {
            item.apply(vertices);
        }
    }

    pub fn revise(&self, geometry: &mut Geometry) {
        let vertices = geometry.vertices_mut();
        self.apply(vertices);
    }

    pub fn distance(&self, geometry: &Geometry, start: usize, end: usize) -> f64 {
        let a = self.items[start].centroid(geometry);
        let b = self.items[end].centroid(geometry);
        a.distance(&b)
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
    fn test_attributeitem_scale_specific() {

        let mut vertices = vec![
            Vertex::new(1.0,1.0,1.0),
            Vertex::new(-1.0,1.0,1.0),
            Vertex::new(1.0,-1.0,1.0),
            Vertex::new(-1.0,-1.0,1.0),
            Vertex::new(1.0,1.0,-1.0),
            Vertex::new(-1.0,1.0,-1.0),
            Vertex::new(1.0,-1.0,-1.0),
            Vertex::new(-1.0,-1.0,-1.0),
        ];
        
        // scale in all directions
        let mut item = AttributeItem::scale_specific(
            Vector::new(1.0,1.0,1.0),
            [0,1,2]
        );

        // scale by a factor of 2.1
        item.update_magnitude(2.1);

        item.apply(&mut vertices);

        fassert_eq!(vertices[0].x, 2.1);
        fassert_eq!(vertices[0].y, 2.1);
        fassert_eq!(vertices[0].z, 2.1);

        fassert_eq!(vertices[1].x, -2.1);
        fassert_eq!(vertices[1].y, 2.1);
        fassert_eq!(vertices[1].z, 2.1);

        fassert_eq!(vertices[2].x, 2.1);
        fassert_eq!(vertices[2].y, -2.1);
        fassert_eq!(vertices[2].z, 2.1);

        fassert_eq!(vertices[3].x, -1.0);
        fassert_eq!(vertices[3].y, -1.0);
        fassert_eq!(vertices[3].z, 1.0);
    }

    #[test]
    fn test_attributeitem_scale_range() {

        let mut vertices = vec![
            Vertex::new(1.0,1.0,1.0),
            Vertex::new(-1.0,1.0,1.0),
            Vertex::new(1.0,-1.0,1.0),
            Vertex::new(-1.0,-1.0,1.0),
            Vertex::new(1.0,1.0,-1.0),
            Vertex::new(-1.0,1.0,-1.0),
            Vertex::new(1.0,-1.0,-1.0),
            Vertex::new(-1.0,-1.0,-1.0),
        ];
        
        // scale in all directions
        let mut item = AttributeItem::scale_range(
            Vector::new(1.0,1.0,1.0),
            0, 3
        );

        // scale by a factor of 2.1
        item.update_magnitude(2.1);
        item.apply(&mut vertices);

        fassert_eq!(vertices[0].x, 2.1);
        fassert_eq!(vertices[0].y, 2.1);
        fassert_eq!(vertices[0].z, 2.1);

        fassert_eq!(vertices[1].x, -2.1);
        fassert_eq!(vertices[1].y, 2.1);
        fassert_eq!(vertices[1].z, 2.1);

        fassert_eq!(vertices[2].x, 2.1);
        fassert_eq!(vertices[2].y, -2.1);
        fassert_eq!(vertices[2].z, 2.1);

        fassert_eq!(vertices[3].x, -1.0);
        fassert_eq!(vertices[3].y, -1.0);
        fassert_eq!(vertices[3].z, 1.0);
    }

    #[test]
    fn test_attributeitem_scale_all() {

        let mut vertices = vec![
            Vertex::new(1.0,1.0,1.0),
            Vertex::new(-1.0,1.0,1.0),
            Vertex::new(1.0,-1.0,1.0),
            Vertex::new(-1.0,-1.0,1.0),
            Vertex::new(1.0,1.0,-1.0),
            Vertex::new(-1.0,1.0,-1.0),
            Vertex::new(1.0,-1.0,-1.0),
            Vertex::new(-1.0,-1.0,-1.0),
        ];
        
        // scale in all directions
        let mut item = AttributeItem::scale_all(
            Vector::new(1.0,1.0,1.0));

        // scale by a factor of 2.1
        item.update_magnitude(2.1);
        item.apply(&mut vertices);

        fassert_eq!(vertices[0].x, 2.1);
        fassert_eq!(vertices[0].y, 2.1);
        fassert_eq!(vertices[0].z, 2.1);

        fassert_eq!(vertices[1].x, -2.1);
        fassert_eq!(vertices[1].y, 2.1);
        fassert_eq!(vertices[1].z, 2.1);

        fassert_eq!(vertices[2].x, 2.1);
        fassert_eq!(vertices[2].y, -2.1);
        fassert_eq!(vertices[2].z, 2.1);

        fassert_eq!(vertices[3].x, -2.1);
        fassert_eq!(vertices[3].y, -2.1);
        fassert_eq!(vertices[3].z, 2.1);

        fassert_eq!(vertices[4].x, 2.1);
        fassert_eq!(vertices[4].y, 2.1);
        fassert_eq!(vertices[4].z, -2.1);

        fassert_eq!(vertices[5].x, -2.1);
        fassert_eq!(vertices[5].y, 2.1);
        fassert_eq!(vertices[5].z, -2.1);

        fassert_eq!(vertices[6].x, 2.1);
        fassert_eq!(vertices[6].y, -2.1);
        fassert_eq!(vertices[6].z, -2.1);

        fassert_eq!(vertices[7].x, -2.1);
        fassert_eq!(vertices[7].y, -2.1);
        fassert_eq!(vertices[7].z, -2.1);
    }

    #[test]
    fn test_attributeitem_rotate_specific() {

        let mut vertices = vec![
            Vertex::new(1.0,1.0,1.0),
            Vertex::new(-1.0,1.0,1.0),
            Vertex::new(1.0,-1.0,1.0),
            Vertex::new(-1.0,-1.0,1.0),
            Vertex::new(1.0,1.0,-1.0),
            Vertex::new(-1.0,1.0,-1.0),
            Vertex::new(1.0,-1.0,-1.0),
            Vertex::new(-1.0,-1.0,-1.0),
        ];
        
        // rotate in all directions
        let mut item = AttributeItem::rotate_specific(
            Vector::new(1.0,1.0,1.0),
            [0,1,2]
        );

        // rotate by 2.1 radians
        item.update_magnitude(2.1);
        item.apply(&mut vertices);

        // verified by https://matrixcalc.org
        fassert_eq!(vertices[0].x, 1.5538668421853181);
        fassert_eq!(vertices[0].y, -0.7645101457817356);
        fassert_eq!(vertices[0].z, -0.03196988823591829);

        fassert_eq!(vertices[1].x, 1.0441276635260175);
        fassert_eq!(vertices[1].y, 0.859417997123489);
        fassert_eq!(vertices[1].z, -1.0822190760100057);

        fassert_eq!(vertices[2].x, 0.6822910697717299);
        fassert_eq!(vertices[2].y, 0.012157775290099204);
        fassert_eq!(vertices[2].z, 1.591958254669306);
    }

    #[test]
    fn test_attributeitem_rotate_range() {

        let mut vertices = vec![
            Vertex::new(1.0,1.0,1.0),
            Vertex::new(-1.0,1.0,1.0),
            Vertex::new(1.0,-1.0,1.0),
            Vertex::new(-1.0,-1.0,1.0),
            Vertex::new(1.0,1.0,-1.0),
            Vertex::new(-1.0,1.0,-1.0),
            Vertex::new(1.0,-1.0,-1.0),
            Vertex::new(-1.0,-1.0,-1.0),
        ];
        
        // rotate in all directions
        let mut item = AttributeItem::rotate_range(
            Vector::new(1.0,1.0,1.0),
            0,3
        );

        // rotate by 2.1 radians
        item.update_magnitude(2.1);
        item.apply(&mut vertices);

        // verified by https://matrixcalc.org
        fassert_eq!(vertices[0].x, 1.5538668421853181);
        fassert_eq!(vertices[0].y, -0.7645101457817356);
        fassert_eq!(vertices[0].z, -0.03196988823591829);

        fassert_eq!(vertices[1].x, 1.0441276635260175);
        fassert_eq!(vertices[1].y, 0.859417997123489);
        fassert_eq!(vertices[1].z, -1.0822190760100057);

        fassert_eq!(vertices[2].x, 0.6822910697717299);
        fassert_eq!(vertices[2].y, 0.012157775290099204);
        fassert_eq!(vertices[2].z, 1.591958254669306);
    }

    #[test]
    fn test_attributeitem_rotate_all() {

        let mut vertices = vec![
            Vertex::new(1.0,1.0,1.0),
            Vertex::new(-1.0,1.0,1.0),
            Vertex::new(1.0,-1.0,1.0),
            Vertex::new(-1.0,-1.0,1.0),
            Vertex::new(1.0,1.0,-1.0),
            Vertex::new(-1.0,1.0,-1.0),
            Vertex::new(1.0,-1.0,-1.0),
            Vertex::new(-1.0,-1.0,-1.0),
        ];
        
        // rotate in all directions
        let mut item = AttributeItem::rotate_all(
            Vector::new(1.0,1.0,1.0));

        // rotate by 2.1 radians
        item.update_magnitude(2.1);
        item.apply(&mut vertices);

        // verified by https://matrixcalc.org
        fassert_eq!(vertices[0].x, 1.5538668421853181);
        fassert_eq!(vertices[0].y, -0.7645101457817356);
        fassert_eq!(vertices[0].z, -0.03196988823591829);

        fassert_eq!(vertices[1].x, 1.0441276635260175);
        fassert_eq!(vertices[1].y, 0.859417997123489);
        fassert_eq!(vertices[1].z, -1.0822190760100057);

        fassert_eq!(vertices[2].x, 0.6822910697717299);
        fassert_eq!(vertices[2].y, 0.012157775290099204);
        fassert_eq!(vertices[2].z, 1.591958254669306);

        fassert_eq!(vertices[3].x, 0.1725518911124293);
        fassert_eq!(vertices[3].y, 1.6360859181953238);
        fassert_eq!(vertices[3].z, 0.5417090668952189);

        fassert_eq!(vertices[4].x, -0.1725518911124293);
        fassert_eq!(vertices[4].y, -1.6360859181953238);
        fassert_eq!(vertices[4].z, -0.5417090668952189);

        fassert_eq!(vertices[5].x, -0.6822910697717299);
        fassert_eq!(vertices[5].y, -0.012157775290099204);
        fassert_eq!(vertices[5].z, -1.591958254669306);

        fassert_eq!(vertices[6].x, -1.0441276635260175);
        fassert_eq!(vertices[6].y, -0.859417997123489);
        fassert_eq!(vertices[6].z, 1.0822190760100057);

        fassert_eq!(vertices[7].x, -1.5538668421853181);
        fassert_eq!(vertices[7].y, 0.7645101457817356);
        fassert_eq!(vertices[7].z, 0.03196988823591829);

    }

    #[test]
    fn test_attributeitem_translate_specific() {

        let mut vertices = vec![
            Vertex::new(1.0,1.0,1.0),
            Vertex::new(-1.0,1.0,1.0),
            Vertex::new(1.0,-1.0,1.0),
            Vertex::new(-1.0,-1.0,1.0),
            Vertex::new(1.0,1.0,-1.0),
            Vertex::new(-1.0,1.0,-1.0),
            Vertex::new(1.0,-1.0,-1.0),
            Vertex::new(-1.0,-1.0,-1.0),
        ];
        
        // translate in all directions
        let mut item = AttributeItem::translate_specific(
            Vector::new(1.0,1.0,1.0),
            [0,1,2]
        );

        // translate by 2.1
        item.update_magnitude(2.1);
        item.apply(&mut vertices);

        fassert_eq!(vertices[0].x, 3.1);
        fassert_eq!(vertices[0].y, 3.1);
        fassert_eq!(vertices[0].z, 3.1);

        fassert_eq!(vertices[1].x, 1.1);
        fassert_eq!(vertices[1].y, 3.1);
        fassert_eq!(vertices[1].z, 3.1);

        fassert_eq!(vertices[2].x, 3.1);
        fassert_eq!(vertices[2].y, 1.1);
        fassert_eq!(vertices[2].z, 3.1);

        fassert_eq!(vertices[3].x, -1.0);
        fassert_eq!(vertices[3].y, -1.0);
        fassert_eq!(vertices[3].z, 1.0);
    }

    #[test]
    fn test_attributeitem_translate_range() {

        let mut vertices = vec![
            Vertex::new(1.0,1.0,1.0),
            Vertex::new(-1.0,1.0,1.0),
            Vertex::new(1.0,-1.0,1.0),
            Vertex::new(-1.0,-1.0,1.0),
            Vertex::new(1.0,1.0,-1.0),
            Vertex::new(-1.0,1.0,-1.0),
            Vertex::new(1.0,-1.0,-1.0),
            Vertex::new(-1.0,-1.0,-1.0),
        ];
        
        // translate in all directions
        let mut item = AttributeItem::translate_range(
            Vector::new(1.0,1.0,1.0),
            0,3
        );

        // translate by 2.1
        item.update_magnitude(2.1);
        item.apply(&mut vertices);

        fassert_eq!(vertices[0].x, 3.1);
        fassert_eq!(vertices[0].y, 3.1);
        fassert_eq!(vertices[0].z, 3.1);

        fassert_eq!(vertices[1].x, 1.1);
        fassert_eq!(vertices[1].y, 3.1);
        fassert_eq!(vertices[1].z, 3.1);

        fassert_eq!(vertices[2].x, 3.1);
        fassert_eq!(vertices[2].y, 1.1);
        fassert_eq!(vertices[2].z, 3.1);

        fassert_eq!(vertices[3].x, -1.0);
        fassert_eq!(vertices[3].y, -1.0);
        fassert_eq!(vertices[3].z, 1.0);
    }

    #[test]
    fn test_attributeitem_translate_all() {

        let mut vertices = vec![
            Vertex::new(1.0,1.0,1.0),
            Vertex::new(-1.0,1.0,1.0),
            Vertex::new(1.0,-1.0,1.0),
            Vertex::new(-1.0,-1.0,1.0),
            Vertex::new(1.0,1.0,-1.0),
            Vertex::new(-1.0,1.0,-1.0),
            Vertex::new(1.0,-1.0,-1.0),
            Vertex::new(-1.0,-1.0,-1.0),
        ];
        
        // translate in all directions
        let mut item = AttributeItem::translate_all(
            Vector::new(1.0,1.0,1.0));

        // translate by 2.1
        item.update_magnitude(2.1);
        item.apply(&mut vertices);

        fassert_eq!(vertices[0].x, 3.1);
        fassert_eq!(vertices[0].y, 3.1);
        fassert_eq!(vertices[0].z, 3.1);

        fassert_eq!(vertices[1].x, 1.1);
        fassert_eq!(vertices[1].y, 3.1);
        fassert_eq!(vertices[1].z, 3.1);

        fassert_eq!(vertices[2].x, 3.1);
        fassert_eq!(vertices[2].y, 1.1);
        fassert_eq!(vertices[2].z, 3.1);

        fassert_eq!(vertices[3].x, 1.1);
        fassert_eq!(vertices[3].y, 1.1);
        fassert_eq!(vertices[3].z, 3.1);

        fassert_eq!(vertices[4].x, 3.1);
        fassert_eq!(vertices[4].y, 3.1);
        fassert_eq!(vertices[4].z, 1.1);

        fassert_eq!(vertices[5].x, 1.1);
        fassert_eq!(vertices[5].y, 3.1);
        fassert_eq!(vertices[5].z, 1.1);

        fassert_eq!(vertices[6].x, 3.1);
        fassert_eq!(vertices[6].y, 1.1);
        fassert_eq!(vertices[6].z, 1.1);

        fassert_eq!(vertices[7].x, 1.1);
        fassert_eq!(vertices[7].y, 1.1);
        fassert_eq!(vertices[7].z, 1.1);


    }
}
