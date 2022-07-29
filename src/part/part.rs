use crate::geometry::*;
use crate::part::*;

#[derive(Default,Debug)]
pub struct Part {
    name: String,
    geometry: Geometry,
    // attributes: Vec<Attribute>,
    connections: Vec<Connection>,
    metadata: Metadata,
}

impl Part {

    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn with_geometry(mut self, geometry: Geometry) -> Self {
        self.geometry = geometry;
        self
    }

    // pub fn with_attribute(mut self, attribute: Attribute) -> Self {
    //     self.attributes.push(attribute);
    //     self
    // }

    pub fn with_connection(mut self, connection: Connection) -> Self {
        self.connections.push(connection);
        self
    }

    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn build(mut self) -> Self {
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
    use crate::models;

    #[test]
    fn test_attribute_transforms_geometry() {
        let mut geometry = models::M2X4.clone();

        let mut length = Attribute::new("Length".into(),vec![
            // translate front end of 2x4 frontwards
            AttributeItem::translate_specific(Vector::new(1.0,0.0,0.0),vec![4,5,6,7]),
            // translate back end of 2x4 backwards
            AttributeItem::translate_specific(Vector::new(-1.0,0.0,0.0),vec![0,1,2,3])
        ]);

        // add 2 meters to the front and back
        length.update(2.0);
        length.revise(&mut geometry);

        let result = length.distance(&geometry,0,1);
        dbg!(result);
    }

    #[test]
    fn test_part_create() {
        let part = Part::new("2x4")
            .with_geometry(models::M2X4.clone())
            .build();
    }

}