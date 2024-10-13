use crate::{ObjProvider, ScriptProvider};

pub struct CacheProvider {
    pub scripts: ScriptProvider,
    pub objs: ObjProvider,
}

impl CacheProvider {
    pub fn new(dir: &str, members: bool) -> CacheProvider {
        return CacheProvider {
            scripts: ScriptProvider::io(dir),
            objs: ObjProvider::io(dir, members),
        };
    }
}
