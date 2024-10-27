use crate::coord_grid::CoordGrid;
use crate::entity::block_walk::BlockWalk;
use crate::entity::entity::Entity;
use crate::entity::lifetime::EntityLifetime;
use crate::entity::move_restrict::MoveRestrict;
use crate::entity::move_strategy::MoveStrategy;

pub struct Npc {
    pub entity: Entity,
    pub move_restrict: MoveRestrict,
    pub block_walk: BlockWalk,
    pub move_strategy: MoveStrategy,
    pub nid: i32,
    pub id: u16, // cache type
}

impl Npc {
    pub fn new(
        coord: CoordGrid,
        width: u8,
        length: u8,
        lifetime: EntityLifetime,
        nid: i32,
        id: u16,
        move_restrict: MoveRestrict,
        block_walk: BlockWalk,
    ) -> Npc {
        Npc {
            entity: Entity {
                coord,
                width,
                length,
                lifetime,
            },
            move_restrict,
            block_walk,
            move_strategy: MoveStrategy::Naive,
            nid,
            id,
        }
    }
}
