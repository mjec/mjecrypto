mod magic_numbers;
#[cfg(test)]
mod tests;

// A struct for holding the current state of a SHA256 calculation in progress
#[derive(Copy, Clone)]
struct HashState {
    result: [u32; 8],
    message_schedule: [u32; 64],
}

/// Methods for calculating SHA256
impl HashState {
    fn new() -> HashState {
        HashState {
            result: magic_numbers::INITIAL_VALUE,
            message_schedule: [0; 64],
        }
    }

    fn update(&mut self, chunk: &[u32; 16]) {
        let mut a: u32 = self.result[0];
        let mut b: u32 = self.result[1];
        let mut c: u32 = self.result[2];
        let mut d: u32 = self.result[3];
        let mut e: u32 = self.result[4];
        let mut f: u32 = self.result[5];
        let mut g: u32 = self.result[6];
        let mut h: u32 = self.result[7];
        self.message_schedule[0..16].copy_from_slice(chunk);

        // Stretching our input across the message schedule
        for j in 16..64 {
            self.message_schedule[j] = little_sigma_1(self.message_schedule[j - 2])
                .wrapping_add(self.message_schedule[j - 7])
                .wrapping_add(little_sigma_0(self.message_schedule[j - 15]))
                .wrapping_add(self.message_schedule[j - 16]);
        }

        // 64 rounds of mixing
        for j in 0..64 {
            let t1 = h
                .wrapping_add(big_sigma_1(e))
                .wrapping_add(ch(e, f, g))
                .wrapping_add(magic_numbers::K[j])
                .wrapping_add(self.message_schedule[j]);
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

        // Store the results
        self.result[0] = a.wrapping_add(self.result[0]);
        self.result[1] = b.wrapping_add(self.result[1]);
        self.result[2] = c.wrapping_add(self.result[2]);
        self.result[3] = d.wrapping_add(self.result[3]);
        self.result[4] = e.wrapping_add(self.result[4]);
        self.result[5] = f.wrapping_add(self.result[5]);
        self.result[6] = g.wrapping_add(self.result[6]);
        self.result[7] = h.wrapping_add(self.result[7]);
    }

    fn result_bytes(&self) -> [u8; 32] {
        let mut output: [u8; 32] = [0; 32];
        output[0..4].copy_from_slice(&self.result[0].to_be_bytes());
        output[4..8].copy_from_slice(&self.result[1].to_be_bytes());
        output[8..12].copy_from_slice(&self.result[2].to_be_bytes());
        output[12..16].copy_from_slice(&self.result[3].to_be_bytes());
        output[16..20].copy_from_slice(&self.result[4].to_be_bytes());
        output[20..24].copy_from_slice(&self.result[5].to_be_bytes());
        output[24..28].copy_from_slice(&self.result[6].to_be_bytes());
        output[28..32].copy_from_slice(&self.result[7].to_be_bytes());
        output
    }
}

/// Hash some bytes with SHA256; output is a string of bytes, always big endian.
///
/// ```rust
/// use mjecrypto::hash::sha256::hash;
/// assert_eq!(
///     hash(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
///     [
///         0x24, 0x8d, 0x6a, 0x61, 0xd2, 0x06, 0x38, 0xb8, 0xe5, 0xc0, 0x26, 0x93, 0x0c, 0x3e, 0x60, 0x39,
///         0xa3, 0x3c, 0xe4, 0x59, 0x64, 0xff, 0x21, 0x67, 0xf6, 0xec, 0xed, 0xd4, 0x19, 0xdb, 0x06, 0xc1,
///     ]
/// );
/// assert_eq!(
///     hash(b"Hello world"),
///     [
///         0x64, 0xec, 0x88, 0xca, 0x00, 0xb2, 0x68, 0xe5, 0xba, 0x1a, 0x35, 0x67, 0x8a, 0x1b, 0x53, 0x16,
///         0xd2, 0x12, 0xf4, 0xf3, 0x66, 0xb2, 0x47, 0x72, 0x32, 0x53, 0x4a, 0x8a, 0xec, 0xa3, 0x7f, 0x3c,
///     ]
/// );
/// ```
pub fn hash(input: &[u8]) -> [u8; 32] {
    let mut state: HashState = HashState::new();

    let mut input_iter = input.chunks_exact(64);

    loop {
        match input_iter.next() {
            None => break,
            Some(chunk_u8) => state.update(&make_16xu32s_from_64xu8s(chunk_u8)),
        }
    }

    // Okay so now we have to pull out the remaining bits, plus whatever padding we need...
    // It's super annoying how complex this is, but it's at most two calls to state.update()
    // so I guess that's acceptable?
    let remainder = input_iter.remainder();
    let (padding_bytes, padding_byte_count) = get_padding_bytes(input.len() * 8);
    let mut last_chunk_u8: [u8; 64] = [0; 64];
    last_chunk_u8[0..remainder.len()].copy_from_slice(remainder);
    let bytes_of_padding_required: usize = 64 - remainder.len();
    last_chunk_u8[remainder.len()..64]
        .copy_from_slice(&padding_bytes[0..bytes_of_padding_required]);
    state.update(&make_16xu32s_from_64xu8s(&last_chunk_u8));
    if padding_byte_count > bytes_of_padding_required {
        last_chunk_u8.copy_from_slice(&padding_bytes[bytes_of_padding_required..]);
        state.update(&make_16xu32s_from_64xu8s(&last_chunk_u8));
    }

    state.result_bytes()
}

/// Returns a 72-byte array filled with padding bytes, and a length which is the number of bytes
/// of that slice that you will need. The remaining bytes will be set to 0x00. By doing this, we
/// avoid using a vec which improves performance a little. Yes, it's possible for padding length
/// to be up to 72 bytes.
#[inline]
fn get_padding_bytes(input_length_in_bits: usize) -> ([u8; 72], usize) {
    debug_assert_eq!(input_length_in_bits % 8, 0);

    let mut output: [u8; 72] = [
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    // number of zeros to get to a multiple of 512 bits total length, given
    // we have to add a 1 and the length of the message in bits, as a big-endian
    // u64.
    let input_length_mod_512 = input_length_in_bits % 512;
    let number_of_zeros: usize =
        447 + if input_length_mod_512 > 447 { 512 } else { 0 } - input_length_mod_512;

    let number_of_padding_bytes: usize = ((number_of_zeros + 1) / 8) + 8;

    output[number_of_padding_bytes - 8] = ((input_length_in_bits >> (7 * 8)) & 0xff) as u8;
    output[number_of_padding_bytes - 7] = ((input_length_in_bits >> (6 * 8)) & 0xff) as u8;
    output[number_of_padding_bytes - 6] = ((input_length_in_bits >> (5 * 8)) & 0xff) as u8;
    output[number_of_padding_bytes - 5] = ((input_length_in_bits >> (4 * 8)) & 0xff) as u8;
    output[number_of_padding_bytes - 4] = ((input_length_in_bits >> (3 * 8)) & 0xff) as u8;
    output[number_of_padding_bytes - 3] = ((input_length_in_bits >> (2 * 8)) & 0xff) as u8;
    output[number_of_padding_bytes - 2] = ((input_length_in_bits >> 8) & 0xff) as u8;
    output[number_of_padding_bytes - 1] = ((input_length_in_bits) & 0xff) as u8;

    (output, number_of_padding_bytes)
}

#[inline]
fn make_16xu32s_from_64xu8s(u8s: &[u8]) -> [u32; 16] {
    debug_assert_eq!(u8s.len(), 64);
    [
        u32::from_be_bytes([u8s[0], u8s[1], u8s[2], u8s[3]]),
        u32::from_be_bytes([u8s[4], u8s[5], u8s[6], u8s[7]]),
        u32::from_be_bytes([u8s[8], u8s[9], u8s[10], u8s[11]]),
        u32::from_be_bytes([u8s[12], u8s[13], u8s[14], u8s[15]]),
        u32::from_be_bytes([u8s[16], u8s[17], u8s[18], u8s[19]]),
        u32::from_be_bytes([u8s[20], u8s[21], u8s[22], u8s[23]]),
        u32::from_be_bytes([u8s[24], u8s[25], u8s[26], u8s[27]]),
        u32::from_be_bytes([u8s[28], u8s[29], u8s[30], u8s[31]]),
        u32::from_be_bytes([u8s[32], u8s[33], u8s[34], u8s[35]]),
        u32::from_be_bytes([u8s[36], u8s[37], u8s[38], u8s[39]]),
        u32::from_be_bytes([u8s[40], u8s[41], u8s[42], u8s[43]]),
        u32::from_be_bytes([u8s[44], u8s[45], u8s[46], u8s[47]]),
        u32::from_be_bytes([u8s[48], u8s[49], u8s[50], u8s[51]]),
        u32::from_be_bytes([u8s[52], u8s[53], u8s[54], u8s[55]]),
        u32::from_be_bytes([u8s[56], u8s[57], u8s[58], u8s[59]]),
        u32::from_be_bytes([u8s[60], u8s[61], u8s[62], u8s[63]]),
    ]
}

/// SHA256 primative
#[inline]
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

/// SHA256 primative
#[inline]
fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

/// SHA256 primative
#[inline]
fn big_sigma_0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

/// SHA256 primative
#[inline]
fn big_sigma_1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

/// SHA256 primative
#[inline]
fn little_sigma_0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

/// SHA256 primative
#[inline]
fn little_sigma_1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}
