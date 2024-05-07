use super::QoS;
use core::{
    ops::{BitAnd, Range},
    str::{from_utf8, Utf8Error},
};

#[derive(Debug)]
pub struct PublishBuilder<const N: usize> {
    buf: [u8; N],
}

impl<const N: usize> PublishBuilder<N> {
    const _PUBLISH_LEN_ASSERT: () = if N < 4 {
        ::core::panic!("Publish packet length is too short")
    };

    pub const fn new() -> Self {
        let mut p = Self { buf: [0; N] };
        p.buf[0] = (super::CtrlPkt::PUBLISH as u8) << 4;
        p.buf[1] = 2;
        p
    }

    pub const fn set_dup(mut self, dup: bool) -> Self {
        if dup {
            self.buf[0] |= 0b1000;
        } else {
            self.buf[0] &= !0b1000;
        }
        self
    }

    pub const fn set_qos(mut self, qos: QoS) -> Self {
        self.buf[0] &= !0b110;
        self.buf[0] |= (qos as u8) << 1;
        self
    }

    pub const fn set_retain(mut self, retain: bool) -> Self {
        if retain {
            self.buf[0] |= 0b1;
        } else {
            self.buf[0] &= !0b1;
        }
        self
    }

    const fn len(&self) -> usize {
        (self.buf[1] + 2) as usize
    }

    pub const fn set_topic(mut self, topic: &str) -> Self {
        let topic_len: u8 = topic.len() as u8;
        self.buf[1] += topic_len;
        self.buf[2] = 0;
        self.buf[3] = topic_len;
        let topic_bytes: &[u8] = topic.as_bytes();
        let len: usize = topic_bytes.len();
        let mut idx: usize = 0;
        loop {
            self.buf[idx + 4] = topic_bytes[idx];
            idx += 1;
            if idx >= len {
                break;
            }
        }
        self
    }

    pub const fn finalize(self) -> Publish<N> {
        Publish { bldr: self }
    }
}

impl<const N: usize> Default for PublishBuilder<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Publish<const N: usize> {
    bldr: PublishBuilder<N>,
}

impl<const N: usize> Publish<N> {
    pub fn as_slice(&self) -> &[u8] {
        &self.bldr.buf[..self.bldr.len()]
    }
}

impl<const N: usize> core::fmt::Write for Publish<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        s.bytes().for_each(|byte| {
            self.bldr.buf[self.bldr.len()] = byte;
            self.bldr.buf[1] += 1;
        });
        Ok(())
    }
}

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
        let first_byte: u8 = *buf.first()?;
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
