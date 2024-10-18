use cache::ScriptPlayer;

#[derive(Clone)]
pub struct Player {
    pub gender: u8,
    pub mask: i32,
    pub anim_id: i32,
    pub anim_delay: i32,
    pub anim_protect: bool,
    pub bas_readyanim: i32,
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

    pub fn new() -> Player {
        return Player {
            gender: 0,
            mask: 0,
            anim_id: -1,
            anim_delay: -1,
            anim_protect: false,
            bas_readyanim: -1,
        };
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
