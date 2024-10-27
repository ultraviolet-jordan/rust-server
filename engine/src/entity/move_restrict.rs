// https://x.com/JagexAsh/status/1678810351091974159
#[repr(u8)]
pub enum MoveRestrict {
    Normal = 0,
    Blocked = 1,
    BlockedNormal = 2,
    Indoors = 3,
    Outdoors = 4,
    NoMove = 5,
    PassThru = 6,
}
