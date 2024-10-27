use packet::out::outgoing::ZoneMessage;

#[repr(u8)]
pub enum ZoneEventType {
    Enclosed = 0,
    Follows = 1,
}

pub struct ZoneEvent {
    pub zone_event_type: ZoneEventType,
    pub receiver_id: i32,
    pub message: Box<dyn ZoneMessage>,
}
