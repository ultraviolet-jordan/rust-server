use crate::{MapProvider, ObjProvider, ScriptProvider};

pub struct CacheProvider {
    pub script_provider: ScriptProvider,
    pub obj_provider: ObjProvider,
    pub map_provider: MapProvider,
}

impl CacheProvider {
    pub fn io(dir: &str, compiler_version: String, members: bool) -> CacheProvider {
        return CacheProvider {
            script_provider: ScriptProvider::io(dir, compiler_version),
            obj_provider: ObjProvider::io(dir, members),
            map_provider: MapProvider::io(dir),
        };
    }

    pub fn mock() -> CacheProvider {
        return CacheProvider {
            script_provider: ScriptProvider::mock(),
            obj_provider: ObjProvider::mock(),
            map_provider: MapProvider::mock(),
        };
    }
}
