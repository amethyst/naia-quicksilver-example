use nanoserde::{DeBin, SerBin};

use naia_derive::Entity;
use naia_shared::{Entity, Property};

use crate::ExampleEntity;

#[derive(Clone, PartialEq, DeBin, SerBin)]
pub enum PointEntityColor {
    Red,
    Blue,
    Yellow,
}

impl Default for PointEntityColor {
    fn default() -> Self { PointEntityColor::Red }
}

#[derive(Entity)]
#[type_name = "ExampleEntity"]
pub struct PointEntity {
    #[interpolate] #[predict] pub x: Property<u16>,
    #[interpolate] #[predict] pub y: Property<u16>,
    pub color: Property<PointEntityColor>
}

impl PointEntity {
    pub fn new(x: u16, y: u16, color: PointEntityColor) -> PointEntity {
        return PointEntity::new_complete(
            x,
            y,
            color,
        );
    }
}
