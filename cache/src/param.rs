use std::collections::HashMap;

use io::Packet;

pub enum ParamValue {
    Int(i32),
    String(String),
}

pub fn decode_params(buf: &mut Packet, params: &mut HashMap<i32, ParamValue>) {
    let count: u8 = buf.g1();
    for _ in 0..count {
        let key: i32 = buf.g3();
        let is_string: bool = buf.g1() == 1;
        if is_string {
            params.insert(key, ParamValue::String(buf.gjstr(10)));
        } else {
            params.insert(key, ParamValue::Int(buf.g4s()));
        }
    }
}
