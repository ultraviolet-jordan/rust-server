use crate::{ObjProvider, ScriptProvider};

pub struct CacheProvider {
    pub script_provider: ScriptProvider,
    pub obj_provider: ObjProvider,
}

impl CacheProvider {
    pub fn new(dir: &str, members: bool) -> CacheProvider {
        return CacheProvider {
            script_provider: ScriptProvider::io(dir),
            obj_provider: ObjProvider::io(dir, members),
        };
    }

    pub fn mock() -> CacheProvider {
        return CacheProvider {
            script_provider: ScriptProvider::mock(),
            obj_provider: ObjProvider::mock(),
        };
    }
}
