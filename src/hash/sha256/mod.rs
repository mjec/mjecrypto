mod magic_numbers;
#[cfg(test)]
mod tests;

/// Hash some bytes with SHA256; output is always big endian.
///
/// ```rust
/// use mjecrypto::hash::sha256::hash;
/// assert_eq!(
///     hash(
///         &String::from("abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")
///             .as_bytes()
///     ),
///     [
///         0x24, 0x8d, 0x6a, 0x61, 0xd2, 0x06, 0x38, 0xb8, 0xe5, 0xc0, 0x26, 0x93, 0x0c, 0x3e,
///         0x60, 0x39, 0xa3, 0x3c, 0xe4, 0x59, 0x64, 0xff, 0x21, 0x67, 0xf6, 0xec, 0xed, 0xd4,
///         0x19, 0xdb, 0x06, 0xc1
///     ]
/// );
/// ```

#[allow(clippy::needless_range_loop)] // this is a false positive
pub fn hash(input: &[u8]) -> [u8; 32] {
    let mut result: [u32; 8] = magic_numbers::INITIAL_VALUE;
    let mut message_schedule: [u32; 64] = [0; 64];
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;
    let mut d: u32;
    let mut e: u32;
    let mut f: u32;
    let mut g: u32;
    let mut h: u32;

    for chunk_u8 in padded_input(input).chunks(64) {
        let chunk: [u32; 16] = make_u32s_from_u8s(chunk_u8);
        message_schedule[0..16].copy_from_slice(&chunk);
        a = result[0];
        b = result[1];
        c = result[2];
        d = result[3];
        e = result[4];
        f = result[5];
        g = result[6];
        h = result[7];

        for j in 16..64 {
            message_schedule[j] = little_sigma_1(message_schedule[j - 2])
                .wrapping_add(message_schedule[j - 7])
                .wrapping_add(little_sigma_0(message_schedule[j - 15]))
                .wrapping_add(message_schedule[j - 16]);
        }

        for j in 0..64 {
            let t1 = h
                .wrapping_add(big_sigma_1(e))
                .wrapping_add(ch(e, f, g))
                .wrapping_add(magic_numbers::K[j])
                .wrapping_add(message_schedule[j]);
            let t2 = big_sigma_0(a).wrapping_add(maj(a, b, c));
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        result[0] = a.wrapping_add(result[0]);
        result[1] = b.wrapping_add(result[1]);
        result[2] = c.wrapping_add(result[2]);
        result[3] = d.wrapping_add(result[3]);
        result[4] = e.wrapping_add(result[4]);
        result[5] = f.wrapping_add(result[5]);
        result[6] = g.wrapping_add(result[6]);
        result[7] = h.wrapping_add(result[7]);
    }

    let mut output: [u8; 32] = [0; 32];
    output[0..4].copy_from_slice(&result[0].to_be_bytes());
    output[4..8].copy_from_slice(&result[1].to_be_bytes());
    output[8..12].copy_from_slice(&result[2].to_be_bytes());
    output[12..16].copy_from_slice(&result[3].to_be_bytes());
    output[16..20].copy_from_slice(&result[4].to_be_bytes());
    output[20..24].copy_from_slice(&result[5].to_be_bytes());
    output[24..28].copy_from_slice(&result[6].to_be_bytes());
    output[28..32].copy_from_slice(&result[7].to_be_bytes());
    output
}

#[inline]
fn make_u32s_from_u8s(chunk_u8: &[u8]) -> [u32; 16] {
    debug_assert_eq!(chunk_u8.len(), 64);
    [
        u32::from_be_bytes([chunk_u8[0], chunk_u8[1], chunk_u8[2], chunk_u8[3]]),
        u32::from_be_bytes([chunk_u8[4], chunk_u8[5], chunk_u8[6], chunk_u8[7]]),
        u32::from_be_bytes([chunk_u8[8], chunk_u8[9], chunk_u8[10], chunk_u8[11]]),
        u32::from_be_bytes([chunk_u8[12], chunk_u8[13], chunk_u8[14], chunk_u8[15]]),
        u32::from_be_bytes([chunk_u8[16], chunk_u8[17], chunk_u8[18], chunk_u8[19]]),
        u32::from_be_bytes([chunk_u8[20], chunk_u8[21], chunk_u8[22], chunk_u8[23]]),
        u32::from_be_bytes([chunk_u8[24], chunk_u8[25], chunk_u8[26], chunk_u8[27]]),
        u32::from_be_bytes([chunk_u8[28], chunk_u8[29], chunk_u8[30], chunk_u8[31]]),
        u32::from_be_bytes([chunk_u8[32], chunk_u8[33], chunk_u8[34], chunk_u8[35]]),
        u32::from_be_bytes([chunk_u8[36], chunk_u8[37], chunk_u8[38], chunk_u8[39]]),
        u32::from_be_bytes([chunk_u8[40], chunk_u8[41], chunk_u8[42], chunk_u8[43]]),
        u32::from_be_bytes([chunk_u8[44], chunk_u8[45], chunk_u8[46], chunk_u8[47]]),
        u32::from_be_bytes([chunk_u8[48], chunk_u8[49], chunk_u8[50], chunk_u8[51]]),
        u32::from_be_bytes([chunk_u8[52], chunk_u8[53], chunk_u8[54], chunk_u8[55]]),
        u32::from_be_bytes([chunk_u8[56], chunk_u8[57], chunk_u8[58], chunk_u8[59]]),
        u32::from_be_bytes([chunk_u8[60], chunk_u8[61], chunk_u8[62], chunk_u8[63]]),
    ]
}

#[inline]
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

#[inline]
fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

#[inline]
fn big_sigma_0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

#[inline]
fn big_sigma_1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

#[inline]
fn little_sigma_0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

#[inline]
fn little_sigma_1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

#[inline]
fn padded_input(input: &[u8]) -> Vec<u8> {
    let input_length_in_bits = input.len() * 8;
    let padding = get_padding_bytes(input_length_in_bits as u64);
    let padding_iter = padding.iter();
    let input_iter = input.iter();
    let padded_input_chain = input_iter.chain(padding_iter);
    padded_input_chain.cloned().collect::<Vec<u8>>()
}

#[inline]
fn get_padding_bytes(input_length_in_bits: u64) -> Vec<u8> {
    // number of zeros to get to a multiple of 512 bits total length, given
    // we have to add a 1 and the length of the message in bits, as a big-endian
    // u64.
    let mut number_of_zeros: i16 = 447 - ((input_length_in_bits % 512) as i16);
    if number_of_zeros < 0 {
        number_of_zeros += 512;
    }

    let mut padding: Vec<u8> = Vec::with_capacity((((number_of_zeros + 1) / 8) + 8) as usize);

    padding.push(1 << 7);

    padding.append(&mut vec![0u8; ((number_of_zeros - 7) / 8) as usize]);

    for shift in (0..=7).rev() {
        padding.push(((input_length_in_bits >> (shift * 8)) & 0xff) as u8);
    }

    padding
}
