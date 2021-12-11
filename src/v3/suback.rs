use core::slice::Iter;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SubCode {
    MaxQoS1 = 0x00,
    MaxQoS2 = 0x01,
    MaxQoS3 = 0x02,
    Failure = 0x80,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SubError {
    /// Packet length is too short.
    Underflow,
    /// Control packet type in the header is not a SUBACK.
    InvalidCtrlPkt(u8),
    /// SUBACK code is invalid.
    InvalidCode(u8),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SubackResult {
    pkt_id: u16,
    code: SubCode,
}

impl SubackResult {
    pub fn from_buf(buf: &[u8]) -> Result<Self, SubError> {
        const EXPECTED_BYTE_1: u8 = (super::CtrlPkt::SUBACK as u8) << 4;

        let mut bytes: Iter<u8> = buf.iter();

        let byte1: u8 = *bytes.next().ok_or(SubError::Underflow)?;
        if byte1 != EXPECTED_BYTE_1 {
            return Err(SubError::InvalidCtrlPkt(byte1));
        }

        let _remain_len: u8 = *bytes.next().ok_or(SubError::Underflow)?;

        let pkt_id: u16 = {
            let pkt_id_msb: u8 = *bytes.next().ok_or(SubError::Underflow)?;
            let pkt_id_lsb: u8 = *bytes.next().ok_or(SubError::Underflow)?;
            u16::from_be_bytes([pkt_id_msb, pkt_id_lsb])
        };

        let code: u8 = *bytes.next().ok_or(SubError::Underflow)?;

        let code: SubCode = match code {
            x if x == SubCode::MaxQoS1 as u8 => Ok(SubCode::MaxQoS1),
            x if x == SubCode::MaxQoS2 as u8 => Ok(SubCode::MaxQoS2),
            x if x == SubCode::MaxQoS3 as u8 => Ok(SubCode::MaxQoS3),
            x if x == SubCode::Failure as u8 => Ok(SubCode::Failure),
            x => Err(SubError::InvalidCode(x)),
        }?;

        Ok(Self { pkt_id, code })
    }

    pub const fn pkt_id(&self) -> u16 {
        self.pkt_id
    }

    pub const fn code(&self) -> SubCode {
        self.code
    }
}
