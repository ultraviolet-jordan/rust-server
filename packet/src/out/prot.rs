pub struct ServerProt {
    pub id: u8,
    pub length: i8,
}

impl ServerProt {
    // interfaces
    pub const IF_OPENCHAT: ServerProt = ServerProt { id: 14, length: 2 };

    // zone protocol
    pub const LOC_ADD_CHANGE: ServerProt = ServerProt { id: 59, length: 4 }; // NXT naming
    pub const MAP_ANIM: ServerProt = ServerProt { id: 191, length: 6 }; // NXT naming
}
