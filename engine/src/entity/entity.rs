use crate::coord_grid::CoordGrid;
use crate::entity::lifetime::EntityLifetime;

#[derive(Eq, Hash, PartialEq)]
pub struct Entity {
    pub coord: CoordGrid,
    pub width: u8,
    pub length: u8,
    pub lifetime: EntityLifetime,
}

pub trait PathingEntity {}

pub trait NonPathingEntity {}
