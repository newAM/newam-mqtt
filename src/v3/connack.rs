use core::{ops::BitAnd, slice::Iter};

/// Connection return codes.
///
/// [Table 3.1 – Connect Return code values](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Table_3.1_-)
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectCode {
    /// Connection Accepted
    Accept = 0,
    /// Connection Refused, unacceptable protocol version
    BadProto = 1,
    /// Connection Refused, identifier rejected
    BadId = 2,
    /// Connection Refused, Server unavailable
    Unavailable = 3,
    /// Connection Refused, bad user name or password
    BadCreds = 4,
    /// Connection Refused, not authorized
    NotAuth = 5,
}

impl ConnectCode {
    /// Returns `true` if the connect code is [`Accept`].
    ///
    /// [`Accept`]: ConnectCode::Accept
    pub const fn is_accept(&self) -> bool {
        matches!(self, Self::Accept)
    }
}

impl From<ConnectCode> for u8 {
    #[inline]
    fn from(val: ConnectCode) -> Self {
        val as u8
    }
}

impl TryFrom<u8> for ConnectCode {
    type Error = u8;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            x if x == ConnectCode::Accept as u8 => Ok(ConnectCode::Accept),
            x if x == ConnectCode::BadProto as u8 => Ok(ConnectCode::BadProto),
            x if x == ConnectCode::BadId as u8 => Ok(ConnectCode::BadId),
            x if x == ConnectCode::Unavailable as u8 => Ok(ConnectCode::Unavailable),
            x if x == ConnectCode::BadCreds as u8 => Ok(ConnectCode::BadCreds),
            x if x == ConnectCode::NotAuth as u8 => Ok(ConnectCode::NotAuth),
            x => Err(x),
        }
    }
}

pub const CONNACK_LEN: usize = 4;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectError {
    /// Packet length is too short.
    Underflow,
    /// Control packet type in the header is not a CONNACK.
    InvalidCtrlPkt(u8),
    /// Remaining length in the packet is invalid.
    InvalidLen(u8),
    /// Connect code is invalid.
    InvalidCode(u8),
}

impl core::fmt::Display for ConnectError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ConnectError::Underflow => write!(f, "packet length is too short"),
            ConnectError::InvalidCtrlPkt(x) => write!(f, "packet type is not CONNACK: {:#02X}", x),
            ConnectError::InvalidLen(x) => write!(f, "remaining length is invalid: {:#02X}", x),
            ConnectError::InvalidCode(x) => write!(f, "CONNACK code is invalid: {:#02X}", x),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ConnectError {}

pub struct ConnackResult {
    session_present: bool,
    code: ConnectCode,
}

impl ConnackResult {
    pub fn from_buf(buf: &[u8]) -> Result<Self, ConnectError> {
        const EXPECTED_BYTE_1: u8 = (super::CtrlPkt::CONNACK as u8) << 4;
        const EXPECTED_BYTE_2: u8 = 2;

        let mut bytes: Iter<u8> = buf.iter();

        let byte1: u8 = *bytes.next().ok_or(ConnectError::Underflow)?;
        if byte1 != EXPECTED_BYTE_1 {
            return Err(ConnectError::InvalidCtrlPkt(byte1));
        }

        let byte2: u8 = *bytes.next().ok_or(ConnectError::Underflow)?;
        if byte2 != EXPECTED_BYTE_2 {
            return Err(ConnectError::InvalidLen(byte2));
        }

        let session_present: bool = bytes
            .next()
            .ok_or(ConnectError::Underflow)?
            .bitand(0b1)
            .eq(&0b1);

        let code: ConnectCode =
            ConnectCode::try_from(*bytes.next().ok_or(ConnectError::Underflow)?)
                .map_err(ConnectError::InvalidCode)?;

        Ok(Self {
            session_present,
            code,
        })
    }

    pub const fn session_present(&self) -> bool {
        self.session_present
    }

    pub const fn code(&self) -> ConnectCode {
        self.code
    }
}
