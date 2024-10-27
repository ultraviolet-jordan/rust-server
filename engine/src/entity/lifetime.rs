#[repr(u8)]
pub enum EntityLifetime {
    Forever = 0, // never respawns or despawns, is always in the world.
    Respawn = 1, // entity added from engine that respawns later.
    Despawn = 2, // entity added from script that despawns later.
}
