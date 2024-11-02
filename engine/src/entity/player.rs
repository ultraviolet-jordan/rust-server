use std::cell::{RefCell, RefMut};

use crate::entity::block_walk::BlockWalk;
use crate::entity::entity::Entity;
use crate::entity::lifetime::EntityLifetime;
use crate::entity::move_restrict::MoveRestrict;
use crate::entity::move_strategy::MoveStrategy;
use crate::grid::coord_grid::CoordGrid;
use cache::{ScriptEngine, ScriptExecutionState, ScriptPlayer, ScriptRunner, ScriptState};
use io::Packet;
use packet::out::outgoing::{OutgoingMessage, ZoneMessage};
use packet::out::priority::ServerProtPriority;

pub struct Player {
    // perm
    pub entity: Entity,
    pub move_restrict: MoveRestrict,
    pub block_walk: BlockWalk,
    pub move_strategy: MoveStrategy,
    pub gender: u8,
    // temp
    pub uid: i32,
    pub mask: i32,
    pub anim_id: i32,
    pub anim_delay: i32,
    pub anim_protect: bool,
    pub bas_readyanim: i32,
    pub active_script: Option<ScriptState>,
}

impl Player {
    const APPEARANCE: i32 = 0x1;
    const ANIM: i32 = 0x2;
    const FACE_ENTITY: i32 = 0x4;
    const SAY: i32 = 0x8;
    const DAMAGE: i32 = 0x10;
    const FACE_COORD: i32 = 0x20;
    const CHAT: i32 = 0x40;
    const BIG_UPDATE: i32 = 0x80;
    const SPOTANIM: i32 = 0x100;
    const EXACT_MOVE: i32 = 0x200;

    pub fn new(coord: CoordGrid, gender: u8) -> Player {
        return Player {
            entity: Entity {
                coord,
                width: 1,
                length: 1,
                lifetime: EntityLifetime::Forever,
            },
            move_restrict: MoveRestrict::Normal,
            block_walk: BlockWalk::Npc,
            move_strategy: MoveStrategy::Smart,
            gender,
            uid: -1,
            mask: 0,
            anim_id: -1,
            anim_delay: -1,
            anim_protect: false,
            bas_readyanim: -1,
            active_script: None,
        };
    }

    pub fn mock() -> Player {
        return Player {
            entity: Entity {
                coord: CoordGrid::new(0),
                width: 1,
                length: 1,
                lifetime: EntityLifetime::Forever,
            },
            move_restrict: MoveRestrict::Normal,
            block_walk: BlockWalk::Npc,
            move_strategy: MoveStrategy::Smart,
            gender: 0,
            uid: -1,
            mask: 0,
            anim_id: -1,
            anim_delay: -1,
            anim_protect: false,
            bas_readyanim: -1,
            active_script: None,
        };
    }

    pub fn resume_script(cell: &RefCell<Player>, engine: &(impl ScriptEngine + ScriptRunner)) {
        let mut player: RefMut<Player> = cell.borrow_mut();
        if let Some(mut state) = player.active_script.take() {
            // just testing
            // state.pc = 0; // reset program.
            // state.push_int(69); // bas_anim id.
            // state.active_player = 0; // active player uid.

            drop(player); // drop the borrow before running.
            let result: Result<(), String> = state.execute(engine, true);

            match result {
                Ok(()) => {
                    let mut player: RefMut<Player> = cell.borrow_mut(); // reborrow and check.
                    if state.execution_state == ScriptExecutionState::Running {
                        player.active_script = Some(state); // put it back on the player.
                    }
                }
                Err(s) => println!("{}", s),
            }
        }
    }

    pub fn write_message(&self, message: &dyn OutgoingMessage) {
        let prio: ServerProtPriority = message.priority();
        let mut buf: Packet = Packet::new(6);
        message.encode(&mut buf);
        println!("{:?}, {:?}", prio, buf.data);
    }

    pub fn write_zone_message(&self, message: &dyn ZoneMessage) {
        let prio: ServerProtPriority = message.priority();
        let bytes: Vec<u8> = message.enclose();
        println!("{:?}, {:?}", prio, bytes);
    }
}

impl ScriptPlayer for Player {
    fn get_gender(&self) -> u8 {
        return self.gender;
    }

    fn play_animation(&mut self, seq: i32, delay: i32) {
        // TODO: other checks after cache loading
        if self.anim_protect {
            return;
        }
        if seq == -1 || self.anim_id == -1 {
            self.anim_id = seq;
            self.anim_delay = delay;
            self.mask |= Player::ANIM;
        }
    }

    fn set_bas_readyanim(&mut self, seq: i32) {
        self.bas_readyanim = seq;
    }
}
