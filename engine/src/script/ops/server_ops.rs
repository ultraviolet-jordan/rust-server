use cache::{ScriptEngine, ScriptOpcode, ScriptState};

use crate::coordgrid::CoordGrid;

pub struct ServerOps;

impl ServerOps {
    pub fn new() -> ServerOps {
        return ServerOps;
    }

    pub fn push(
        &self,
        engine: &impl ScriptEngine,
        state: &mut ScriptState,
        code: &ScriptOpcode,
    ) -> Result<(), String> {
        return match code {
            ScriptOpcode::CoordX => self.coord_x(state),
            ScriptOpcode::CoordY => self.coord_y(state),
            ScriptOpcode::CoordZ => self.coord_z(state),
            ScriptOpcode::Distance => self.distance(state),
            ScriptOpcode::HuntAll => Err("Not implemented".to_string()),
            ScriptOpcode::HuntNext => Err("Not implemented".to_string()),
            ScriptOpcode::InZone => Err("Not implemented".to_string()),
            ScriptOpcode::LineOfSight => self.line_of_sight(engine, state),
            ScriptOpcode::LineOfWalk => self.line_of_walk(engine, state),
            ScriptOpcode::MapBlocked => Err("Not implemented".to_string()),
            ScriptOpcode::MapIndoors => Err("Not implemented".to_string()),
            ScriptOpcode::MapClock => self.map_clock(engine, state),
            ScriptOpcode::MapLocAddUnsafe => Err("Not implemented".to_string()),
            ScriptOpcode::MapMembers => self.map_members(engine, state),
            ScriptOpcode::MapPlayerCount => Err("Not implemented".to_string()),
            ScriptOpcode::MapFindSquare => Err("Not implemented".to_string()),
            ScriptOpcode::MoveCoord => Err("Not implemented".to_string()),
            ScriptOpcode::PlayerCount => Err("Not implemented".to_string()),
            ScriptOpcode::ProjAnimMap => Err("Not implemented".to_string()),
            ScriptOpcode::ProjAnimNpc => Err("Not implemented".to_string()),
            ScriptOpcode::ProjAnimPl => Err("Not implemented".to_string()),
            ScriptOpcode::SeqLength => Err("Not implemented".to_string()),
            ScriptOpcode::SplitGet => Err("Not implemented".to_string()),
            ScriptOpcode::SplitGetAnim => Err("Not implemented".to_string()),
            ScriptOpcode::SplitInit => Err("Not implemented".to_string()),
            ScriptOpcode::SplitLineCount => Err("Not implemented".to_string()),
            ScriptOpcode::SplitPageCount => Err("Not implemented".to_string()),
            ScriptOpcode::SpotAnimMap => Err("Not implemented".to_string()),
            ScriptOpcode::StatRandom => Err("Not implemented".to_string()),
            ScriptOpcode::StructParam => Err("Not implemented".to_string()),
            ScriptOpcode::WorldDelay => Err("Not implemented".to_string()),
            ScriptOpcode::NpcsCount => Err("Not implemented".to_string()),
            ScriptOpcode::ZonesCount => Err("Not implemented".to_string()),
            ScriptOpcode::LocsCount => Err("Not implemented".to_string()),
            ScriptOpcode::ObjsCount => Err("Not implemented".to_string()),
            ScriptOpcode::MapMulti => Err("Not implemented".to_string()),
            _ => Err(format!("Unrecognised server ops code: {:?}", code)),
        };
    }

    #[inline(always)]
    fn coord_x(&self, state: &mut ScriptState) -> Result<(), String> {
        let coord: CoordGrid = CoordGrid::new(state.pop_int() as u32);
        state.push_int(coord.x() as i32);
        return Ok(());
    }

    #[inline(always)]
    fn coord_y(&self, state: &mut ScriptState) -> Result<(), String> {
        let coord: CoordGrid = CoordGrid::new(state.pop_int() as u32);
        state.push_int(coord.y() as i32);
        return Ok(());
    }

    #[inline(always)]
    fn coord_z(&self, state: &mut ScriptState) -> Result<(), String> {
        let coord: CoordGrid = CoordGrid::new(state.pop_int() as u32);
        state.push_int(coord.z() as i32);
        return Ok(());
    }

    #[inline(always)]
    fn distance(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: CoordGrid = CoordGrid::new(state.pop_int() as u32);
        let a: CoordGrid = CoordGrid::new(state.pop_int() as u32);
        state.push_int(a.distance(b) as i32);
        return Ok(());
    }

    #[inline(always)]
    fn line_of_sight(
        &self,
        engine: &impl ScriptEngine,
        state: &mut ScriptState,
    ) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(engine.line_of_sight(a, b) as i32);
        return Ok(());
    }

    #[inline(always)]
    fn line_of_walk(
        &self,
        engine: &impl ScriptEngine,
        state: &mut ScriptState,
    ) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(engine.line_of_walk(a, b) as i32);
        return Ok(());
    }

    #[inline(always)]
    fn map_clock(&self, engine: &impl ScriptEngine, state: &mut ScriptState) -> Result<(), String> {
        state.push_int(engine.map_clock() as i32);
        return Ok(());
    }

    #[inline(always)]
    fn map_members(
        &self,
        engine: &impl ScriptEngine,
        state: &mut ScriptState,
    ) -> Result<(), String> {
        state.push_int(engine.map_members() as i32);
        return Ok(());
    }
}
