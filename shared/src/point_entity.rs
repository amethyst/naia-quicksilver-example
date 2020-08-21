use nanoserde::{DeBin, SerBin};

use naia_derive::Entity;
use naia_shared::{Entity, Property};

use crate::ExampleEntity;

#[derive(Entity)]
#[type_name = "ExampleEntity"]
pub struct PointEntity {
    pub x: Property<u16>,
    pub y: Property<u16>,
}

impl PointEntity {
    pub fn new(x: u16, y: u16) -> PointEntity {
        return PointEntity::new_complete(
            x,
            y,
        );
    }
}
