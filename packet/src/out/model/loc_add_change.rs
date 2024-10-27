use crate::out::outgoing::{OutgoingMessage, ZoneMessage};
use crate::out::priority::ServerProtPriority;
use crate::out::prot::ServerProt;
use io::Packet;

#[derive(Eq, Hash, PartialEq)]
pub struct LocAddChange {
    pub coord: u8,
    pub id: u16,
    pub shape: u8,
    pub angle: u8,
}

impl OutgoingMessage for LocAddChange {
    fn priority(&self) -> ServerProtPriority {
        return ServerProtPriority::Immediate;
    }

    fn prot(&self) -> ServerProt {
        return ServerProt::LOC_ADD_CHANGE;
    }

    fn encode(&self, buf: &mut Packet) {
        buf.p1(self.coord as i32);
        buf.p1(((self.shape << 2) | (self.angle & 0x3)) as i32);
        buf.p2(self.id as i32);
    }
}

impl ZoneMessage for LocAddChange {}
