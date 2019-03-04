use super::*;

fn assert_padding_is_correct(
    expected_padding_size_bytes: usize,
    unpadded_length_bits: u64,
    actual_padding_tuple: ([u8; 72], usize),
) {
    let (actual_padding_bytes, actual_padding_size_bytes) = actual_padding_tuple;
    let mut actual_length_in_bits_u8: [u8; 8] = [0; 8];
    actual_length_in_bits_u8.copy_from_slice(
        &actual_padding_bytes[expected_padding_size_bytes - 8..expected_padding_size_bytes],
    );
    assert_eq!(
        actual_padding_size_bytes, expected_padding_size_bytes,
        "Actual padding size in bytes (left) should equal expected padding size in bytes (right)"
    );
    assert_eq!(
        actual_padding_bytes[0], 0x80,
        "First byte of padding must be 0x80"
    );
    assert_eq!(
        u64::from_be_bytes(actual_length_in_bits_u8),
        unpadded_length_bits,
        "Last 8 bytes of padding decoded as u64 should give unpadded length in bits"
    );
    for b in actual_padding_bytes[1..expected_padding_size_bytes - 8].iter() {
        assert_eq!(*b, 0, "Padding bytes should be 0");
    }
}

#[test]
fn test_vectors() {
    assert_padding_is_correct(61, 3 * 8, get_padding_bytes(3 * 8));
    assert_eq!(
        hash(b"abc"),
        [
            0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
            0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
            0xf2, 0x00, 0x15, 0xad
        ],
        "Hash of abc should be ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );

    assert_padding_is_correct(72, 56 * 8, get_padding_bytes(56 * 8));
    assert_eq!(
        hash(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
        [
            0x24, 0x8d, 0x6a, 0x61, 0xd2, 0x06, 0x38, 0xb8, 0xe5, 0xc0, 0x26, 0x93, 0x0c, 0x3e,
            0x60, 0x39, 0xa3, 0x3c, 0xe4, 0x59, 0x64, 0xff, 0x21, 0x67, 0xf6, 0xec, 0xed, 0xd4,
            0x19, 0xdb, 0x06, 0xc1
        ],
        "Hash of abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq should be 248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
    )
}

#[test]
fn test_padding() {
    for len in 0..=512 {
        let padding_tuple = get_padding_bytes(len * 8);
        assert_eq!(
            ((padding_tuple.1 * 8) + (len * 8)) % 512,
            0,
            "Length of padded value in bits must be a multiple of 512"
        );
        // We use padding_tuple.1 as expected_padding_size_bytes to avoid duplicating
        // calcuations from get_padding_bytes() here (which wouldn't really test anything).
        // There are a couple of tests for the accuracy of this in test_vectors().
        assert_padding_is_correct(padding_tuple.1, len as u64 * 8, padding_tuple);
    }
}
