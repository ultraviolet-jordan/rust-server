#[repr(u8)]
#[derive(Eq, Hash, PartialEq)]
pub enum ZoneEventType {
    Enclosed = 0,
    Follows = 1,
}

#[derive(Eq, Hash, PartialEq)]
pub struct ZoneEvent {
    pub zone_event_type: ZoneEventType,
    pub receiver_id: i32,
}
