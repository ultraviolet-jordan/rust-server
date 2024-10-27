use crate::coord_grid::CoordGrid;
use crate::entity::lifetime::EntityLifetime;

pub struct Entity {
    pub coord: CoordGrid,
    pub width: u8,
    pub length: u8,
    pub lifetime: EntityLifetime,
}
