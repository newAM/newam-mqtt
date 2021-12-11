//! MQTT v3.1.1
//!
//! See the [specification] for details.
//!
//! [specification]: (http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html)

mod connack;
mod connect;
mod suback;

pub use connack::{ConnackResult, ConnectCode, ConnectError, CONNACK_LEN};
pub use connect::Connect;
pub use suback::{SubCode, SubError, SubackResult};

/// Control packet types.
///
/// See [Table 2.1 - Control packet types](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Table_2.1_-)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum CtrlPkt {
    CONNECT = 0x1,
    CONNACK = 0x2,
    PUBLISH = 0x3,
    PUBACK = 0x4,
    PUBREC = 0x5,
    PUBREL = 0x6,
    PUBCOMP = 0x7,
    SUBSCRIBE = 0x8,
    SUBACK = 0x9,
    UNSUBSCRIBE = 0xA,
    UNSUBACK = 0xB,
    PINGREQ = 0xC,
    PINGRESP = 0xD,
    DISCONNECT = 0xE,
}

impl From<CtrlPkt> for u8 {
    fn from(ctrl_pkt: CtrlPkt) -> Self {
        ctrl_pkt as u8
    }
}

impl TryFrom<u8> for CtrlPkt {
    type Error = u8;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            x if x == CtrlPkt::CONNECT as u8 => Ok(CtrlPkt::CONNECT),
            x if x == CtrlPkt::CONNACK as u8 => Ok(CtrlPkt::CONNACK),
            x if x == CtrlPkt::PUBLISH as u8 => Ok(CtrlPkt::PUBLISH),
            x if x == CtrlPkt::PUBACK as u8 => Ok(CtrlPkt::PUBACK),
            x if x == CtrlPkt::PUBREC as u8 => Ok(CtrlPkt::PUBREC),
            x if x == CtrlPkt::PUBREL as u8 => Ok(CtrlPkt::PUBREL),
            x if x == CtrlPkt::PUBCOMP as u8 => Ok(CtrlPkt::PUBCOMP),
            x if x == CtrlPkt::SUBSCRIBE as u8 => Ok(CtrlPkt::SUBSCRIBE),
            x if x == CtrlPkt::SUBACK as u8 => Ok(CtrlPkt::SUBACK),
            x if x == CtrlPkt::UNSUBSCRIBE as u8 => Ok(CtrlPkt::UNSUBSCRIBE),
            x if x == CtrlPkt::UNSUBACK as u8 => Ok(CtrlPkt::UNSUBACK),
            x if x == CtrlPkt::PINGREQ as u8 => Ok(CtrlPkt::PINGREQ),
            x if x == CtrlPkt::PINGRESP as u8 => Ok(CtrlPkt::PINGRESP),
            x if x == CtrlPkt::DISCONNECT as u8 => Ok(CtrlPkt::DISCONNECT),
            x => Err(x),
        }
    }
}
