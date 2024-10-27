use crate::out::outgoing::{OutgoingMessage, ZoneMessage};
use crate::out::priority::ServerProtPriority;
use crate::out::prot::ServerProt;
use io::Packet;

pub struct MapAnim {
    pub coord: u8,
    pub spotanim: u16,
    pub height: i32,
    pub delay: u32,
}

impl OutgoingMessage for MapAnim {
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    fn prot(&self) -> ServerProt {
        return ServerProt::MAP_ANIM;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.coord as i32);
        buf.p2(self.spotanim as i32);
        buf.p1(self.height);
        buf.p2(self.delay as i32);
    }
}

impl ZoneMessage for MapAnim {}
