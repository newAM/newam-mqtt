use super::QoS;
use core::{
    ops::{BitAnd, Range},
    str::{from_utf8, Utf8Error},
};

#[derive(Debug)]
pub struct PublishDe<'a> {
    pub dup_flag: bool,
    pub qos_level: Option<QoS>,
    pub retain: bool,
    pub topic: Result<&'a str, Utf8Error>,
    pub payload: &'a [u8],
}

impl<'a> PublishDe<'a> {
    pub fn new(buf: &'a [u8]) -> Option<Self> {
        let first_byte: u8 = *buf.get(0)?;
        if first_byte >> 4 & 0x0F == u8::from(super::CtrlPkt::PUBLISH) {
            let remain_len: u8 = *buf.get(1)?;
            let topic_len: u16 = u16::from_be_bytes(buf.get(2..4)?.try_into().unwrap());

            let topic_range: Range<usize> = Range {
                start: 4,
                end: 4_usize.saturating_add(topic_len as usize),
            };

            let payload_len: usize = remain_len
                .saturating_sub(u8::try_from(topic_len).unwrap_or(u8::MAX))
                .saturating_sub(2)
                .into();
            let payload_range: Range<usize> = Range {
                start: topic_range.end,
                end: topic_range.end.saturating_add(payload_len),
            };

            let topic_bytes: &[u8] = buf.get(topic_range)?;
            let payload_bytes: &[u8] = buf.get(payload_range)?;

            Some(Self {
                dup_flag: first_byte.bitand(0b1000).eq(&0b1000),
                qos_level: QoS::try_from(first_byte >> 1 & 0b11).ok(),
                retain: first_byte.bitand(0b1).eq(&0b1),
                topic: from_utf8(topic_bytes),
                payload: payload_bytes,
            })
        } else {
            None
        }
    }

    pub fn payload_utf8(&self) -> Result<&str, Utf8Error> {
        from_utf8(self.payload)
    }
}
