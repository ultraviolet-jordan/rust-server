use cache::ScriptPlayer;

#[derive(Clone)]
pub struct Player {
    pub bas_readyanim: i32,
}

impl Player {
    pub fn new() -> Player {
        return Player { bas_readyanim: -1 };
    }
}

impl ScriptPlayer for Player {
    fn get_gender(&self) -> u8 {
        return 0; // TODO
    }

    fn set_bas_readyanim(&mut self, seq: i32) {
        self.bas_readyanim = seq;
    }
}
