use std::collections::HashMap;

use io::Packet;

pub enum ScriptOpcode {
    // Core language ops (0-99)
    PushConstantInt = 0,    // official, see cs2
    PushVarp = 1,           // official, see cs2
    PopVarp = 2,            // official, see cs2
    PushConstantString = 3, // official, see cs2
    PushVarn = 4,
    PopVarn = 5,
    Branch = 6,             // official, see cs2
    BranchNot = 7,          // official, see cs2
    BranchEquals = 8,       // official, see cs2
    BranchLessThan = 9,     // official, see cs2
    BranchGreaterThan = 10, // official, see cs2
    PushVars = 11,
    PopVars = 12,
    Return = 21, // official, see cs2
    GoSub = 22,
    Jump = 23,
    Switch = 24,
    PushVarbit = 25,
    PopVarbit = 26,
    BranchLessThanOrEquals = 31,    // official, see cs2
    BranchGreaterThanOrEquals = 32, // official, see cs2
    PushIntLocal = 33,              // official, see cs2
    PopIntLocal = 34,               // official, see cs2
    PushStringLocal = 35,           // official, see cs2
    PopStringLocal = 36,            // official, see cs2
    JoinString = 37,                // official, see cs2
    PopIntDiscard = 38,             // official, see cs2
    PopStringDiscard = 39,          // official, see cs2
    GoSubWithParams = 40,           // official, see cs2
    JumpWithParams = 41,            // official, see cs2
    PushVarcInt = 42,
    PopVarcInt = 43,
    DefineArray = 44,  // official, see cs2
    PushArrayInt = 45, // official, see cs2
    PopArrayInt = 46,  // official, see cs2

                       // Server ops (1000-1999)
                       // Player ops (2000-2499)
                       // Npc ops (2500-2999)
                       // Loc ops (3000-3499)
                       // Obj ops (3500-4000)
                       // Npc config ops (4000-4099)
                       // Loc config ops (4100-4199)
                       // Obj config ops (4200-4299)
                       // Inventory ops (4300-4399)
                       // Enum ops (4400-4499)
                       // String ops (4500-4599)
                       // Number ops (4600-4699)
                       // DB ops (7500-7599)
                       // Debug ops (10000-11000)
}

impl From<u16> for ScriptOpcode {
    fn from(code: u16) -> ScriptOpcode {
        match code {
            // Core language ops (0-99)
            0 => ScriptOpcode::PushConstantInt,
            1 => ScriptOpcode::PushVarp,
            2 => ScriptOpcode::PopVarp,
            3 => ScriptOpcode::PushConstantString,
            4 => ScriptOpcode::PushVarn,
            5 => ScriptOpcode::PopVarn,
            6 => ScriptOpcode::Branch,
            7 => ScriptOpcode::BranchNot,
            8 => ScriptOpcode::BranchEquals,
            9 => ScriptOpcode::BranchLessThan,
            10 => ScriptOpcode::BranchGreaterThan,
            11 => ScriptOpcode::PushVars,
            12 => ScriptOpcode::PopVars,
            21 => ScriptOpcode::Return,
            22 => ScriptOpcode::GoSub,
            23 => ScriptOpcode::Jump,
            24 => ScriptOpcode::Switch,
            25 => ScriptOpcode::PushVarbit,
            26 => ScriptOpcode::PopVarbit,
            31 => ScriptOpcode::BranchLessThanOrEquals,
            32 => ScriptOpcode::BranchGreaterThanOrEquals,
            33 => ScriptOpcode::PushIntLocal,
            34 => ScriptOpcode::PopIntLocal,
            35 => ScriptOpcode::PushStringLocal,
            36 => ScriptOpcode::PopStringLocal,
            37 => ScriptOpcode::JoinString,
            38 => ScriptOpcode::PopIntDiscard,
            39 => ScriptOpcode::PopStringDiscard,
            40 => ScriptOpcode::GoSubWithParams,
            41 => ScriptOpcode::JumpWithParams,
            42 => ScriptOpcode::PushVarcInt,
            43 => ScriptOpcode::PopVarcInt,
            44 => ScriptOpcode::DefineArray,
            45 => ScriptOpcode::PushArrayInt,
            46 => ScriptOpcode::PopArrayInt,
            // Server ops (1000-1999)
            // Player ops (2000-2499)
            // Npc ops (2500-2999)
            // Loc ops (3000-3499)
            // Obj ops (3500-4000)
            // Npc config ops (4000-4099)
            // Loc config ops (4100-4199)
            // Obj config ops (4200-4299)
            // Inventory ops (4300-4399)
            // Enum ops (4400-4499)
            // String ops (4500-4599)
            // Number ops (4600-4699)
            // DB ops (7500-7599)
            // Debug ops (10000-11000)
            _ => panic!("Invalid script opcode value: {}", code),
        }
    }
}

pub struct ScriptProvider {
    names: HashMap<String, usize>,
    scripts: Vec<ScriptFile>,
    lookups: HashMap<i32, usize>,
}

impl ScriptProvider {
    pub fn io(dir: &str) -> ScriptProvider {
        let mut dat: Packet = Packet::io(format!("{}/server/script.dat", dir));
        let mut idx: Packet = Packet::io(format!("{}/server/script.idx", dir));

        let count: usize = dat.g2() as usize;
        idx.pos += 2;

        let version: i32 = dat.g4s();
        let compiler: &String = &std::env::var("COMPILER_VERSION").unwrap();
        if !version.to_string().eq(compiler) {
            panic!("RuneScript compiler is out of date!");
        }

        let mut names: HashMap<String, usize> = HashMap::new();
        let mut scripts: Vec<ScriptFile> = Vec::with_capacity(count);
        let mut lookups: HashMap<i32, usize> = HashMap::new();

        for id in 0..count {
            let size: usize = idx.g2() as usize;
            if size == 0 {
                continue;
            }

            let mut data: Vec<u8> = vec![0; size];
            dat.gdata(&mut data, 0, size);
            let mut script: ScriptFile = ScriptFile::new(id);
            script.decode(Packet::from(data));

            if let Some(info) = &script.info {
                names.insert(info.name.clone(), id);
                // add the script to lookup table if the value isn't -1
                if info.lookup != -1 {
                    lookups.insert(info.lookup, id);
                }
            }
            scripts.push(script);
        }

        return ScriptProvider {
            names,
            scripts,
            lookups,
        };
    }

    pub fn get_by_id<F, E>(&self, id: usize, on_found: F, on_not_found: E)
    where
        F: FnOnce(&ScriptFile),
        E: FnOnce(),
    {
        if let Some(obj) = self.scripts.get(id) {
            on_found(obj);
        } else {
            on_not_found();
        }
    }

    pub fn get_by_name<F, E>(&self, name: &str, on_found: F, on_not_found: E)
    where
        F: FnOnce(&ScriptFile),
        E: FnOnce(),
    {
        if let Some(id) = self.names.get(name) {
            self.get_by_id(*id, on_found, on_not_found);
        } else {
            on_not_found();
        }
    }

    pub fn get_by_trigger<F, E>(&self, on_found: F, on_not_found: E)
    where
        F: FnOnce(&ScriptFile),
        E: FnOnce(),
    {
        // TODO: use the lookup to get the script id then get script by id.
        // TODO: instead of it existing here, probs let consumer write their own.
    }

    pub fn get_by_trigger_specific<F, E>(&self, on_found: F, on_not_found: E)
    where
        F: FnOnce(&ScriptFile),
        E: FnOnce(),
    {
        // TODO: use the lookup to get the script id then get script by id.
        // TODO: instead of it existing here, probs let consumer write their own.
    }
}

pub struct ScriptInfo {
    pub name: String,
    pub path: String,
    pub lookup: i32,
    pub params: Vec<u8>,
    pub pcs: Vec<i32>,
    pub lines: Vec<i32>,
}

pub struct ScriptFile {
    id: usize,
    int_local_count: u16,
    string_local_count: u16,
    pub int_arg_count: u16,
    pub string_arg_count: u16,
    pub switch_table: Option<HashMap<i32, i32>>,
    pub info: Option<ScriptInfo>,
    pub codes: Option<Vec<u16>>,
    pub int_operands: Option<Vec<i32>>,
    pub string_operands: Option<Vec<Option<String>>>,
}

impl ScriptFile {
    fn new(id: usize) -> ScriptFile {
        return ScriptFile {
            id,
            int_local_count: 0,
            string_local_count: 0,
            int_arg_count: 0,
            string_arg_count: 0,
            switch_table: None,
            info: None,
            codes: None,
            int_operands: None,
            string_operands: None,
        };
    }

    fn decode(&mut self, mut buf: Packet) {
        let length: usize = buf.len();
        if length < 16 {
            panic!("Invalid script file (minimum length) must be 16 bytes.");
        }

        buf.pos = length - 2;

        let trailer_len: usize = buf.g2() as usize;
        let trailer_pos: usize = length - trailer_len - 12 - 2;

        if trailer_pos >= length {
            panic!("Invalid script file (bad trailer pos).");
        }

        buf.pos = trailer_pos;

        let instructions: usize = buf.g4s() as usize;
        self.int_local_count = buf.g2();
        self.string_local_count = buf.g2();
        self.int_arg_count = buf.g2();
        self.string_arg_count = buf.g2();

        let switches: usize = buf.g1() as usize;
        for _ in 0..switches {
            let count: usize = buf.g2() as usize;
            let mut table: HashMap<i32, i32> = HashMap::new();

            for _ in 0..count {
                table.insert(buf.g4s(), buf.g4s());
            }

            self.switch_table = Some(table);
        }

        buf.pos = 0;
        let name: String = buf.gjstr(0);
        let path: String = buf.gjstr(0);
        let lookup: i32 = buf.g4s();

        let params_count: usize = buf.g1() as usize;
        let mut params: Vec<u8> = vec![0; params_count];
        for index in 0..params_count {
            params[index] = buf.g1();
        }

        let lines_count: usize = buf.g2() as usize;
        let mut pcs: Vec<i32> = vec![0; lines_count];
        let mut lines: Vec<i32> = vec![0; lines_count];
        for index in 0..lines_count {
            pcs[index] = buf.g4s();
            lines[index] = buf.g4s();
        }

        self.info = Some(ScriptInfo {
            name,
            path,
            lookup,
            params,
            pcs,
            lines,
        });

        let mut opcodes: Vec<u16> = vec![0; instructions];
        let mut int_operands: Vec<i32> = vec![0; instructions];
        let mut string_operands: Vec<Option<String>> = vec![None; instructions];

        let mut pc: usize = 0;
        while trailer_pos > buf.pos {
            let code: u16 = buf.g2();

            if code == ScriptOpcode::PushConstantString as u16 {
                string_operands[pc] = Some(buf.gjstr(0));
            } else if ScriptFile::is_large_operand(code) {
                int_operands[pc] = buf.g4s();
            } else {
                int_operands[pc] = buf.g1() as i32;
            }

            opcodes[pc] = code;
            pc += 1;
        }

        self.codes = Some(opcodes);
        self.int_operands = Some(int_operands);
        self.string_operands = Some(string_operands)
    }

    fn is_large_operand(code: u16) -> bool {
        if code > 100 {
            return false;
        }
        return match ScriptOpcode::from(code) {
            ScriptOpcode::Return => false,
            ScriptOpcode::PopIntDiscard => false,
            ScriptOpcode::PopStringDiscard => false,
            ScriptOpcode::GoSub => false,
            ScriptOpcode::Jump => false,
            _ => true,
        };
    }
}
