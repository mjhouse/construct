use crate::geometry::{Matrix,Vertex,Transform};
use crate::constant::Index;
use crate::errors::Error;

#[derive(Debug,Clone)]
pub enum VertexSelection {
    Specific(Vec<Index>),
    Range((Index,Index)),
    All,
}

#[derive(Debug,Clone)]
pub enum VertexTransform {
    Translate,
    Rotate,
    Scale,
}

#[derive(Debug,Clone)]
pub struct AttributeItem {
    selection: VertexSelection,
    transform: VertexTransform,
}

#[derive(Default,Debug,Clone)]
pub struct Attribute {
    scale: f64,
    name:  String,
    items: Vec<AttributeItem>,
}

pub trait ApplyAttribute<T> {
    fn apply(&self, value: T, vertices: &mut Vec<Vertex>);
}

impl Attribute {
    
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            scale: 1.0,
            name:  name.into(),
            items: Vec::new(),
        }
    }

    pub fn with_scale<T: Into<f64>>(mut self, value: T) -> Self {
        self.scale = value.into();
        self
    }

    pub fn with_item(mut self, selection: VertexSelection, transform: VertexTransform) -> Self {
        self.items.push(AttributeItem {
            selection,
            transform,
        });
        self
    }

    pub fn with_specific(mut self, vertices: Vec<Index>, transform: VertexTransform) -> Self {
        self.with_item(VertexSelection::Specific(vertices),transform)
    }

    pub fn with_range(mut self, start: Index, end: Index, transform: VertexTransform) -> Self {
        self.with_item(VertexSelection::Range((start,end)),transform)
    }

    pub fn with_all(mut self, transform: VertexTransform) -> Self {
        self.with_item(VertexSelection::All,transform)
    }

    pub fn build(self) -> Result<Self,Error> {

        if self.scale == 0.0 {
            Err(Error::FixedAttribute)?;
        }

        if self.name.len() == 0 {
            Err(Error::UnnamedAttribute)?;
        }

        if self.items.len() == 0 {
            Err(Error::EmptyAttribute)?;
        }

        Ok(self)
    }

}

impl VertexSelection {

    fn take_specific(&self, indices: Vec<usize>, vertices: &Vec<Vertex>) -> Vec<(Index,Vertex)> {
        let mut result = vec![];
        for index in indices.into_iter() {
            result.push((
                index,
                vertices[index]
            ));
        }
        result
    }

    fn take_range(&self, start: usize, end: usize, vertices: &Vec<Vertex>) -> Vec<(Index,Vertex)> {
        let mut result = vec![];
        for index in start..end {
            result.push((
                index,
                vertices[index]
            ));
        }
        result
    }

    fn take_all(&self, vertices: &Vec<Vertex>) -> Vec<(Index,Vertex)> {
        let mut result = vec![];
        for (index,vertex) in vertices.iter().enumerate() {
            result.push((
                index,
                *vertex
            ));
        }
        result
    }

    pub fn take(&self, vertices: &Vec<Vertex>) -> Vec<(Index,Vertex)> {
        use VertexSelection::*;
        match self {
            Specific(indices)  => self.take_specific(indices.clone(),vertices),
            Range((start,end)) => self.take_range(*start,*end,vertices),
            All                => self.take_all(vertices),
        }
    }
}

impl VertexTransform {
    fn matrix(&self, x: f64, y: f64, z: f64) -> Matrix {
        match self {
            Translate => Matrix::translate(x,y,z),
            Rotate    => Matrix::rotate(x,y,z),
            Scale     => Matrix::scale(x,y,z),
        }
    }
}

impl ApplyAttribute<(f64,f64,f64)> for Attribute {
    fn apply(&self, (x,y,z): (f64,f64,f64), vertices: &mut Vec<Vertex>) {
        for item in self.items.iter() {
            let matrix = item.transform.matrix(x,y,z);
            let select = item.selection.take(vertices);
            for (index, mut vertex) in select.into_iter() {
                vertex.transform(&matrix);
                vertices[index] = vertex;
            }
        }
    }
}

impl ApplyAttribute<f64> for Attribute {
    fn apply(&self, v: f64, vertices: &mut Vec<Vertex>) {
        self.apply((v,v,v),vertices);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::models;

    // #[test]
    // fn test_attribute_create() {
    //     let attribute = Attribute::new("Length")
    //         .with_range(0,3,)
    // }

}