#[test]
#[cfg(feature = "std")]
fn connect() {
    use newam_mqtt::v3::ConnectAlloc;

    const CLIENT_ID: &'static str = "foo";

    let connect: ConnectAlloc = ConnectAlloc::with_client_id(CLIENT_ID);
    assert_eq!(
        connect.as_bytes(),
        &[
            0x10,
            0x0C,
            0,
            4,
            b'M',
            b'Q',
            b'T',
            b'T',
            4,
            2,
            14,
            16,
            0,
            CLIENT_ID.len() as u8,
            b'f',
            b'o',
            b'o'
        ]
    );
}
