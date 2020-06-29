pub fn sha1(data: &[u8]) -> Digest {
    let mut state: [u32; 5] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];
    let mut len: u64 = 0;
    let mut blocks = Blocks {
        len: 0,
        data: [0; 64],
    };
    process_blocks(&mut blocks, data, &mut len, &mut state);
    digest(state, len, blocks)
}

struct Blocks {
    len: u32,
    data: [u8; 64],
}

fn digest(mut state: [u32; 5], len: u64, blocks: Blocks) -> Digest {
    let bits = (len + (blocks.len as u64)) * 8;
    let extra = [
        (bits >> 56) as u8,
        (bits >> 48) as u8,
        (bits >> 40) as u8,
        (bits >> 32) as u8,
        (bits >> 24) as u8,
        (bits >> 16) as u8,
        (bits >> 8) as u8,
        (bits >> 0) as u8,
    ];
    let mut last = [0; 128];
    let blocklen = blocks.len as usize;
    last[..blocklen].clone_from_slice(&blocks.data[..blocklen]);
    last[blocklen] = 0x80;

    if blocklen < 56 {
        last[56..64].clone_from_slice(&extra);
        process_state(&mut state, as_block(&last[0..64]));
    } else {
        last[120..128].clone_from_slice(&extra);
        process_state(&mut state, as_block(&last[0..64]));
        process_state(&mut state, as_block(&last[64..128]));
    }
    Digest { state }
}

fn process_blocks(blocks: &mut Blocks, data: &[u8], len: &mut u64, state: &mut [u32; 5]) {
    for chunk in data.chunks(64) {
        if chunk.len() == 64 {
            let chunk_block = as_block(chunk);
            *len = chunk_block.len() as u64 * 4;
            process_state(state, chunk_block);
        } else {
            blocks.data[..chunk.len()].clone_from_slice(chunk);
            blocks.len = chunk.len() as u32;
        }
    }
}

fn process_state(state: &mut [u32; 5], mut block: [u32; 16]) {
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let block = &mut block;
    r0(block, a, &mut b, c, d, &mut e, 0);
    r0(block, e, &mut a, b, c, &mut d, 1);
    r0(block, d, &mut e, a, b, &mut c, 2);
    r0(block, c, &mut d, e, a, &mut b, 3);
    r0(block, b, &mut c, d, e, &mut a, 4);
    r0(block, a, &mut b, c, d, &mut e, 5);
    r0(block, e, &mut a, b, c, &mut d, 6);
    r0(block, d, &mut e, a, b, &mut c, 7);
    r0(block, c, &mut d, e, a, &mut b, 8);
    r0(block, b, &mut c, d, e, &mut a, 9);
    r0(block, a, &mut b, c, d, &mut e, 10);
    r0(block, e, &mut a, b, c, &mut d, 11);
    r0(block, d, &mut e, a, b, &mut c, 12);
    r0(block, c, &mut d, e, a, &mut b, 13);
    r0(block, b, &mut c, d, e, &mut a, 14);
    r0(block, a, &mut b, c, d, &mut e, 15);
    r1(block, e, &mut a, b, c, &mut d, 0);
    r1(block, d, &mut e, a, b, &mut c, 1);
    r1(block, c, &mut d, e, a, &mut b, 2);
    r1(block, b, &mut c, d, e, &mut a, 3);
    r2(block, a, &mut b, c, d, &mut e, 4);
    r2(block, e, &mut a, b, c, &mut d, 5);
    r2(block, d, &mut e, a, b, &mut c, 6);
    r2(block, c, &mut d, e, a, &mut b, 7);
    r2(block, b, &mut c, d, e, &mut a, 8);
    r2(block, a, &mut b, c, d, &mut e, 9);
    r2(block, e, &mut a, b, c, &mut d, 10);
    r2(block, d, &mut e, a, b, &mut c, 11);
    r2(block, c, &mut d, e, a, &mut b, 12);
    r2(block, b, &mut c, d, e, &mut a, 13);
    r2(block, a, &mut b, c, d, &mut e, 14);
    r2(block, e, &mut a, b, c, &mut d, 15);
    r2(block, d, &mut e, a, b, &mut c, 0);
    r2(block, c, &mut d, e, a, &mut b, 1);
    r2(block, b, &mut c, d, e, &mut a, 2);
    r2(block, a, &mut b, c, d, &mut e, 3);
    r2(block, e, &mut a, b, c, &mut d, 4);
    r2(block, d, &mut e, a, b, &mut c, 5);
    r2(block, c, &mut d, e, a, &mut b, 6);
    r2(block, b, &mut c, d, e, &mut a, 7);
    r3(block, a, &mut b, c, d, &mut e, 8);
    r3(block, e, &mut a, b, c, &mut d, 9);
    r3(block, d, &mut e, a, b, &mut c, 10);
    r3(block, c, &mut d, e, a, &mut b, 11);
    r3(block, b, &mut c, d, e, &mut a, 12);
    r3(block, a, &mut b, c, d, &mut e, 13);
    r3(block, e, &mut a, b, c, &mut d, 14);
    r3(block, d, &mut e, a, b, &mut c, 15);
    r3(block, c, &mut d, e, a, &mut b, 0);
    r3(block, b, &mut c, d, e, &mut a, 1);
    r3(block, a, &mut b, c, d, &mut e, 2);
    r3(block, e, &mut a, b, c, &mut d, 3);
    r3(block, d, &mut e, a, b, &mut c, 4);
    r3(block, c, &mut d, e, a, &mut b, 5);
    r3(block, b, &mut c, d, e, &mut a, 6);
    r3(block, a, &mut b, c, d, &mut e, 7);
    r3(block, e, &mut a, b, c, &mut d, 8);
    r3(block, d, &mut e, a, b, &mut c, 9);
    r3(block, c, &mut d, e, a, &mut b, 10);
    r3(block, b, &mut c, d, e, &mut a, 11);
    r4(block, a, &mut b, c, d, &mut e, 12);
    r4(block, e, &mut a, b, c, &mut d, 13);
    r4(block, d, &mut e, a, b, &mut c, 14);
    r4(block, c, &mut d, e, a, &mut b, 15);
    r4(block, b, &mut c, d, e, &mut a, 0);
    r4(block, a, &mut b, c, d, &mut e, 1);
    r4(block, e, &mut a, b, c, &mut d, 2);
    r4(block, d, &mut e, a, b, &mut c, 3);
    r4(block, c, &mut d, e, a, &mut b, 4);
    r4(block, b, &mut c, d, e, &mut a, 5);
    r4(block, a, &mut b, c, d, &mut e, 6);
    r4(block, e, &mut a, b, c, &mut d, 7);
    r4(block, d, &mut e, a, b, &mut c, 8);
    r4(block, c, &mut d, e, a, &mut b, 9);
    r4(block, b, &mut c, d, e, &mut a, 10);
    r4(block, a, &mut b, c, d, &mut e, 11);
    r4(block, e, &mut a, b, c, &mut d, 12);
    r4(block, d, &mut e, a, b, &mut c, 13);
    r4(block, c, &mut d, e, a, &mut b, 14);
    r4(block, b, &mut c, d, e, &mut a, 15);

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
}

fn rol(value: u32, bits: usize) -> u32 {
    (value << bits) | (value >> (32 - bits))
}

fn blk(block: &[u32; 16], i: usize) -> u32 {
    let value = block[(i + 13) & 15] ^ block[(i + 8) & 15] ^ block[(i + 2) & 15] ^ block[i];
    rol(value, 1)
}

fn r0(block: &mut [u32; 16], v: u32, w: &mut u32, x: u32, y: u32, z: &mut u32, i: usize) {
    let n = ((*w & (x ^ y)) ^ y)
        .wrapping_add(block[i])
        .wrapping_add(0x5a82_7999)
        .wrapping_add(rol(v, 5));
    *z = z.wrapping_add(n);
    *w = rol(*w, 30);
}

fn r1(block: &mut [u32; 16], v: u32, w: &mut u32, x: u32, y: u32, z: &mut u32, i: usize) {
    block[i] = blk(block, i);
    let n = ((*w & (x ^ y)) ^ y)
        .wrapping_add(block[i])
        .wrapping_add(0x5a82_7999)
        .wrapping_add(rol(v, 5));
    *z = z.wrapping_add(n);
    *w = rol(*w, 30);
}

fn r2(block: &mut [u32; 16], v: u32, w: &mut u32, x: u32, y: u32, z: &mut u32, i: usize) {
    block[i] = blk(block, i);
    let n = (*w ^ x ^ y)
        .wrapping_add(block[i])
        .wrapping_add(0x6ed_9eba1)
        .wrapping_add(rol(v, 5));
    *z = z.wrapping_add(n);
    *w = rol(*w, 30);
}

fn r3(block: &mut [u32; 16], v: u32, w: &mut u32, x: u32, y: u32, z: &mut u32, i: usize) {
    block[i] = blk(block, i);
    let n = (((*w | x) & y) | (*w & x))
        .wrapping_add(block[i])
        .wrapping_add(0x8f1b_bcdc)
        .wrapping_add(rol(v, 5));
    *z = z.wrapping_add(n);
    *w = rol(*w, 30);
}

fn r4(block: &mut [u32; 16], v: u32, w: &mut u32, x: u32, y: u32, z: &mut u32, i: usize) {
    block[i] = blk(block, i);
    let n = (*w ^ x ^ y)
        .wrapping_add(block[i])
        .wrapping_add(0xca62_c1d6)
        .wrapping_add(rol(v, 5));
    *z = z.wrapping_add(n);
    *w = rol(*w, 30);
}

pub struct Digest {
    state: [u32; 5],
}

impl std::fmt::Display for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in self.state.iter() {
            write!(f, "{:08x}", i)?;
        }
        Ok(())
    }
}

fn as_block(input: &[u8]) -> [u32; 16] {
    assert!(input.len() == 64);
    let mut result = [0u32; 16];

    for i in 0..16 {
        let off = i * 4;
        result[i] = (input[off + 3] as u32)
            | ((input[off + 2] as u32) << 8)
            | ((input[off + 1] as u32) << 16)
            | ((input[off] as u32) << 24);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let tests = [
            (
                "The quick brown fox jumps over the lazy dog",
                "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12",
            ),
            (
                "The quick brown fox jumps over the lazy cog",
                "de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3",
            ),
            ("", "da39a3ee5e6b4b0d3255bfef95601890afd80709"),
            ("testing\n", "9801739daae44ec5293d4e1f53d3f4d2d426d91c"),
            ("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
             "025ecbd5d70f8fb3c5457cd96bab13fda305dc59"),
            ("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
             "4300320394f7ee239bcdce7d3b8bcee173a0cd5c"),
            ("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
             "cef734ba81a024479e09eb5a75b6ddae62e6abf1"),
        ];

        for &(s, ref h) in tests.iter() {
            let data = s.as_bytes();

            let hh = sha1(data).to_string();

            assert_eq!(hh, *h);
        }
    }
}
