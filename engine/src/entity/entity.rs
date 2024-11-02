use crate::entity::lifetime::EntityLifetime;
use crate::grid::coord_grid::CoordGrid;

#[derive(Eq, Hash, PartialEq)]
pub struct Entity {
    pub coord: CoordGrid,
    pub width: u8,
    pub length: u8,
    pub lifetime: EntityLifetime,
}

pub trait PathingEntity {}

pub trait NonPathingEntity {}
