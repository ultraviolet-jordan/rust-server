use crate::out::priority::ServerProtPriority;
use crate::out::prot::ServerProt;
use io::Packet;
use std::hash::Hash;

pub trait OutgoingMessage {
    fn priority(&self) -> ServerProtPriority;
    fn prot(&self) -> ServerProt;
    fn encode(&self, buf: &mut Packet);

    fn test(&self) -> i8 {
        return self.prot().length;
    }
}

pub trait ZoneMessage: OutgoingMessage {
    fn enclose(&self) -> Vec<u8> {
        let mut buf: Packet = Packet::new(1 + self.prot().length as usize);
        buf.p1(self.prot().id as i32);
        self.encode(&mut buf);
        return buf.data;
    }
}
