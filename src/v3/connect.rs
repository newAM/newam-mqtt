pub struct Connect {
    buf: [u8; Self::LEN],
}

impl Connect {
    pub const LEN: usize = 14;

    const PROTO_LEN: u16 = 4;
    const PROTO_LEVEL: u8 = 4;

    const DEFAULT_KEEP_ALIVE: u16 = 3600;

    pub const DEFAULT: Self = Connect {
        buf: [
            (super::CtrlPkt::CONNECT as u8) << 4,
            (Self::LEN as u8) - 2, // length of packet after this byte
            (Self::PROTO_LEN >> 8) as u8,
            Self::PROTO_LEN as u8,
            b'M',
            b'Q',
            b'T',
            b'T',
            Self::PROTO_LEVEL,
            0x02, // clean session flag (required with no client ID)
            (Self::DEFAULT_KEEP_ALIVE >> 8) as u8,
            Self::DEFAULT_KEEP_ALIVE as u8,
            0, // no client ID
            0, // no client ID
        ],
    };

    #[must_use = "set_keep_alive returns a modified Connect"]
    pub const fn set_keep_alive(mut self, keep_alive: u16) -> Self {
        self.buf[10] = (keep_alive >> 8) as u8;
        self.buf[11] = keep_alive as u8;
        self
    }

    pub const fn into_array(self) -> [u8; Self::LEN] {
        self.buf
    }
}
