use crate::geometry::{Matrix,Vertex,Transform};
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

impl Selection {

    pub fn new<T: Into<Vec<Index>>>(indices: T) -> Self {
        Self::specific(indices)
    }

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

    fn give_specific<T: Clone>(&self, indices: Vec<Index>, src: &Vec<T>, dest: &mut Vec<T>) {

        // TODO: verify that all indices are less than max length
        // TODO: verify that indices.len() == src.len()

        for index in indices.into_iter() {
            dest[index] = src[index].clone();
        }
    }

    fn give_range<T: Clone>(&self, (start,end): (Index,Index), src: &Vec<T>, dest: &mut Vec<T>) {
        // TODO: verify that start is less than end
        // TODO: verify that end is less than max length
        // TODO: verify that end-start is equal to src length

        let target = &mut dest[start..end + 1];
        let source = &src[start..end + 1];

        for (i,item) in source.iter().enumerate() {
            target[i] = item.clone();
        }
    }

    fn give_all<T: Clone>(&self, src: &Vec<T>, dest: &mut Vec<T>) {

        // TODO: verify that src is the same size as dest

        for (i,item) in src.iter().enumerate() {
            dest[i] = item.clone();
        }
    }

    fn take_specific<T: Clone>(&self, indices: Vec<Index>, data: &Vec<T>) -> Vec<T> {
        let mut result = vec![];

        // TODO: verify that all indices are less than max length

        for index in indices.into_iter() {
            result.push(data[index].clone());
        }
        result
    }

    fn take_range<T: Clone>(&self, (start,end): (Index,Index), data: &Vec<T>) -> Vec<T> {
        let mut result = vec![];

        // TODO: verify that start is less than end
        // TODO: verify the end is less than max length

        for item in data[start..end].iter() {
            result.push(item.clone());
        }
        result
    }

    fn take_all<T: Clone>(&self, data: &Vec<T>) -> Vec<T> {
        data.clone()
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::models;

    #[test]
    fn test_selection_create() {
        // just make sure these compile
        let specific = Selection::specific([1, 2]);
        let range = Selection::range(1,2);
        let all = Selection::all();
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
    fn test_selection_give_specific() {
        let src = vec![
            0, 1, 2, 3, 4, 
            5, 6, 7, 8, 9
        ];

        let mut dest = vec![
            9, 8, 7, 6, 5, 
            4, 3, 2, 1, 0
        ];

        let select = Selection::specific([
            0, 1, 5
        ]); // give 1st, 2nd and 5th items

        select.give(&src,&mut dest);

        assert_eq!(dest[0],0);
        assert_eq!(dest[1],1);
        assert_eq!(dest[5],5);
    }

    #[test]
    fn test_selection_take_range() {
        let data = vec![
            9, 8, 7, 6, 5, 
            4, 3, 2, 1, 0
        ];

        let select = Selection::range(
            0, 3
        ); // take 1st, 2nd and 2rd items

        let taken = select.take(&data);

        assert_eq!(taken.len(),3);
        assert_eq!(taken[0],9);
        assert_eq!(taken[1],8);
        assert_eq!(taken[2],7);
    }

    #[test]
    fn test_selection_give_range() {
        let src = vec![
            0, 1, 2, 3, 4, 
            5, 6, 7, 8, 9
        ];

        let mut dest = vec![
            9, 8, 7, 6, 5, 
            4, 3, 2, 1, 0
        ];

        let select = Selection::range(0, 5);
        select.give(&src,&mut dest);

        assert_eq!(&src[0..6],&dest[0..6]);
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

}
















// #[derive(Debug,Clone)]
// pub enum VertexSelection {
//     Specific(Vec<Index>),
//     Range((Index,Index)),
//     All,
// }

// #[derive(Debug,Clone)]
// pub enum VertexTransform {
//     Translate,
//     Rotate,
//     Scale,
// }

// #[derive(Debug,Clone)]
// pub struct AttributeItem {
//     selection: VertexSelection,
//     transform: VertexTransform,
// }

// #[derive(Default,Debug,Clone)]
// pub struct Attribute {
//     scale: f64,
//     name:  String,
//     items: Vec<AttributeItem>,
// }

// pub trait ApplyAttribute<T> {
//     fn apply(&self, value: T, vertices: &mut Vec<Vertex>);
// }

// impl Attribute {
    
//     pub fn new<T: Into<String>>(name: T) -> Self {
//         Self {
//             scale: 1.0,
//             name:  name.into(),
//             items: Vec::new(),
//         }
//     }

//     pub fn with_scale<T: Into<f64>>(mut self, value: T) -> Self {
//         self.scale = value.into();
//         self
//     }

//     pub fn with_item(mut self, selection: VertexSelection, transform: VertexTransform) -> Self {
//         self.items.push(AttributeItem {
//             selection,
//             transform,
//         });
//         self
//     }

//     pub fn with_specific(mut self, vertices: Vec<Index>, transform: VertexTransform) -> Self {
//         self.with_item(VertexSelection::Specific(vertices),transform)
//     }

//     pub fn with_range(mut self, start: Index, end: Index, transform: VertexTransform) -> Self {
//         self.with_item(VertexSelection::Range((start,end)),transform)
//     }

//     pub fn with_all(mut self, transform: VertexTransform) -> Self {
//         self.with_item(VertexSelection::All,transform)
//     }

//     pub fn build(self) -> Result<Self,Error> {

//         if self.scale == 0.0 {
//             Err(Error::FixedAttribute)?;
//         }

//         if self.name.len() == 0 {
//             Err(Error::UnnamedAttribute)?;
//         }

//         if self.items.len() == 0 {
//             Err(Error::EmptyAttribute)?;
//         }

//         Ok(self)
//     }

// }

// impl VertexSelection {

//     fn take_specific(&self, indices: Vec<usize>, vertices: &Vec<Vertex>) -> Vec<(Index,Vertex)> {
//         let mut result = vec![];
//         for index in indices.into_iter() {
//             result.push((
//                 index,
//                 vertices[index]
//             ));
//         }
//         result
//     }

//     fn take_range(&self, start: usize, end: usize, vertices: &Vec<Vertex>) -> Vec<(Index,Vertex)> {
//         let mut result = vec![];
//         for index in start..end {
//             result.push((
//                 index,
//                 vertices[index]
//             ));
//         }
//         result
//     }

//     fn take_all(&self, vertices: &Vec<Vertex>) -> Vec<(Index,Vertex)> {
//         let mut result = vec![];
//         for (index,vertex) in vertices.iter().enumerate() {
//             result.push((
//                 index,
//                 *vertex
//             ));
//         }
//         result
//     }

//     pub fn take(&self, vertices: &Vec<Vertex>) -> Vec<(Index,Vertex)> {
//         use VertexSelection::*;
//         match self {
//             Specific(indices)  => self.take_specific(indices.clone(),vertices),
//             Range((start,end)) => self.take_range(*start,*end,vertices),
//             All                => self.take_all(vertices),
//         }
//     }
// }

// impl VertexTransform {
//     fn matrix(&self, x: f64, y: f64, z: f64) -> Matrix {
//         match self {
//             Translate => Matrix::translate(x,y,z),
//             Rotate    => Matrix::rotate(x,y,z),
//             Scale     => Matrix::scale(x,y,z),
//         }
//     }
// }

// impl ApplyAttribute<(f64,f64,f64)> for Attribute {
//     fn apply(&self, (x,y,z): (f64,f64,f64), vertices: &mut Vec<Vertex>) {
//         for item in self.items.iter() {
//             let matrix = item.transform.matrix(x,y,z);
//             let select = item.selection.take(vertices);
//             for (index, mut vertex) in select.into_iter() {
//                 vertex.transform(&matrix);
//                 vertices[index] = vertex;
//             }
//         }
//     }
// }

// impl ApplyAttribute<f64> for Attribute {
//     fn apply(&self, v: f64, vertices: &mut Vec<Vertex>) {
//         self.apply((v,v,v),vertices);
//     }
// }
