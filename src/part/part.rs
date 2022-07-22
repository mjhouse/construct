use crate::geometry::*;
use crate::part::*;

#[derive(Default,Debug,Clone)]
pub struct Part {
    name: String,
    geometry: Geometry,
    attributes: Vec<Attribute>,
    connections: Vec<Connection>,
    metadata: Metadata,
}

impl Part {

    pub fn new(name: String) -> Self {
        Self {
            name: name,
            ..Default::default()
        }
    }

    pub fn with_geometry(mut self, geometry: Geometry) -> Self {
        self.geometry = geometry;
        self
    }

    pub fn with_attribute(mut self, attribute: Attribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    pub fn with_connection(mut self, connection: Connection) -> Self {
        self.connections.push(connection);
        self
    }

    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn construct(mut self) -> Self {
        /*
            verify:
                1. attributes map to real geometry
                2. connection points are on surface
                3. geometry is not empty
        */
        self
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn test_triangle_normal() {
    //     let data = vec![
    //         Vertex::new(0,0,0),
    //         Vertex::new(1,0,0),
    //         Vertex::new(0,1,0),
    //     ];

    //     let f = Face::new(1,2,3);
    //     let t = f.triangle(&data);
    //     let normal = t.normal();

    //     assert_eq!(normal.x,0.0);
    //     assert_eq!(normal.y,0.0);
    //     assert_eq!(normal.z,1.0);
    // }

}