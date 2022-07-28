use std::iter::zip;
use log;

use crate::geometry::{Matrix,Vector,Vertex,Transform};
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

    pub fn give<T: Clone>(&self, src: &Vec<T>, dest: &mut Vec<T>) {
        match self.clone() {
            Selection::Specific(v) => self.give_specific(v,src,dest),
            Selection::Range(v) => self.give_range(v,src,dest),
            Selection::All => self.give_all(src,dest)
        }
    }

    pub fn take<T: Clone>(&self, data: &Vec<T>) -> Vec<T> {
        match self.clone() {
            Selection::Specific(v) => self.take_specific(v,data),
            Selection::Range(v) => self.take_range(v,data),
            Selection::All => self.take_all(data)
        }
    }

    /// Update specific indices from source
    ///
    /// # Errors
    ///
    /// Will panic if any of the given indives are out of bounds
    /// for the destination.
    fn give_specific<T: Clone>(&self, indices: Vec<Index>, src: &Vec<T>, dest: &mut Vec<T>) {
        for (i,v) in zip(indices.into_iter(),src.iter()) {
            dest[i] = v.clone();
        }
    }


    /// Update a range of indices from source
    ///
    /// # Errors
    ///
    /// Will panic if the end is less than the start or 
    /// the end is out of bounds for the destination.
    fn give_range<T: Clone>(&self, (start,end): (Index,Index), src: &Vec<T>, dest: &mut Vec<T>) {
        dest.splice(start..end, src
            .iter()
            .cloned());
    }


    /// Overwrite the destination from the source
    ///
    /// # Errors
    ///
    /// N/A
    fn give_all<T: Clone>(&self, src: &Vec<T>, dest: &mut Vec<T>) {
        dest.splice(.., src
            .iter()
            .cloned());
    }


    /// Take specific indices from the data
    ///
    /// # Errors
    ///
    /// Will panic if a given index is out of bounds
    /// for data.
    fn take_specific<T: Clone>(&self, indices: Vec<Index>, data: &Vec<T>) -> Vec<T> {
        indices
            .into_iter()
            .map(|i| data[i].clone())
            .collect()
    }


    /// Take a range of elements from the data
    ///
    /// # Errors
    ///
    /// Will panic if the end is less than start or 
    /// the end is out of bounds for data.
    fn take_range<T: Clone>(&self, (start,end): (Index,Index), data: &Vec<T>) -> Vec<T> {
        data[start..end]
            .iter()
            .cloned()
            .collect()
    }

    /// Take all elements from the given data
    ///
    /// # Errors
    ///
    /// N/A
    fn take_all<T: Clone>(&self, data: &Vec<T>) -> Vec<T> {
        data.clone()
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
        // take selection
        let mut points = self.selection.take(vertices);

        // transform selection
        self.alteration.apply(&mut points);

        // return selection
        self.selection.give(&points,vertices);
    }

}

impl Attribute {
    
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
    fn test_selection_take_specific() {
        let data = vec![
            9, 8, 7, 6, 5, 
            4, 3, 2, 1, 0
        ];

        let select = Selection::specific([
            0, 1, 5
        ]); // take 1st, 2nd and 5th items

        let taken = select.take(&data);

        assert_eq!(taken.len(),3);
        assert_eq!(taken[0],9);
        assert_eq!(taken[1],8);
        assert_eq!(taken[2],4);
    }

    #[test]
    fn test_selection_give_specific_start() {
        let src = vec![
            0, 1, 2, 3, 4
        ];

        let mut dest = vec![
            9, 8, 7, 6, 5, 
            4, 3, 2, 1, 0
        ];

        let select = Selection::specific([0,1,2,3,4]);
        select.give(&src,&mut dest);

        assert_eq!(&src[0..5],&dest[0..5]);
    }

    #[test]
    fn test_selection_give_specific_end() {
        let src = vec![
            0, 1, 2, 3, 4
        ];

        let mut dest = vec![
            9, 8, 7, 6, 5, 
            4, 3, 2, 1, 0
        ];

        let select = Selection::specific([5,6,7,8,9]);
        select.give(&src,&mut dest);

        assert_eq!(&src[0..5],&dest[5..10]);
    }

    #[test]
    fn test_selection_take_range_start() {
        let data = vec![
            9, 8, 7, 6, 5, 
            4, 3, 2, 1, 0
        ];

        let select = Selection::range(0, 3);
        let taken = select.take(&data);

        assert_eq!(taken.len(),3);
        assert_eq!(taken[0],9);
        assert_eq!(taken[1],8);
        assert_eq!(taken[2],7);
    }

    #[test]
    fn test_selection_take_range_end() {
        let data = vec![
            9, 8, 7, 6, 5, 
            4, 3, 2, 1, 0
        ];

        let select = Selection::range(7, 10);
        let taken = select.take(&data);

        assert_eq!(taken.len(),3);
        assert_eq!(taken[0],2);
        assert_eq!(taken[1],1);
        assert_eq!(taken[2],0);
    }

    #[test]
    fn test_selection_give_range_start() {
        let src = vec![
            0, 1, 2, 3, 4
        ];

        let mut dest = vec![
            9, 8, 7, 6, 5, 
            4, 3, 2, 1, 0
        ];

        let select = Selection::range(0, 5);
        select.give(&src,&mut dest);

        assert_eq!(&src[0..5],&dest[0..5]);
    }

    #[test]
    fn test_selection_give_range_end() {
        let src = vec![
            0, 1, 2, 3, 4
        ];

        let mut dest = vec![
            9, 8, 7, 6, 5, 
            4, 3, 2, 1, 0
        ];

        let select = Selection::range(5, 10);
        select.give(&src,&mut dest);

        assert_eq!(&src[0..5],&dest[5..10]);
    }

    #[test]
    fn test_selection_take_all() {
        let data = vec![
            9, 8, 7, 6, 5, 
            4, 3, 2, 1, 0
        ];

        let select = Selection::all();
        let taken = select.take(&data);
        assert_eq!(taken,data);
    }

    #[test]
    fn test_selection_give_all() {
        let src = vec![
            0, 1, 2, 3, 4, 
            5, 6, 7, 8, 9
        ];

        let mut dest = vec![
            9, 8, 7, 6, 5, 
            4, 3, 2, 1, 0
        ];

        let select = Selection::all();
        select.give(&src,&mut dest);
        
        assert_eq!(src,dest);
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
