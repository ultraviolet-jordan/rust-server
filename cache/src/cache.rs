use crate::{ObjProvider, ScriptProvider};

pub struct CacheProvider {
    pub scripts: ScriptProvider,
    pub obj_provider: ObjProvider,
}

impl CacheProvider {
    pub fn new(dir: &str, members: bool) -> CacheProvider {
        return CacheProvider {
            scripts: ScriptProvider::io(dir),
            obj_provider: ObjProvider::io(dir, members),
        };
    }

    pub fn mock() -> CacheProvider {
        return CacheProvider {
            scripts: ScriptProvider::mock(),
            obj_provider: ObjProvider::mock(),
        };
    }
}
