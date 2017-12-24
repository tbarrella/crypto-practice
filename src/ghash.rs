use core::ops::{BitXorAssign, MulAssign};
use byteorder::{BigEndian, ByteOrder};

pub(crate) fn ghash(key: &[u8; 16], data: &[u8], ciphertext: &[u8]) -> [u8; 16] {
    let mut tag = [0; 16];
    let mut mac = GHash::new(key, data);
    mac.update(ciphertext);
    mac.write_tag(&mut tag);
    tag
}

const R0: u64 = 0xe1 << 56;

struct GHash {
    function: PolyFunction,
    data_len: u64,
    ciphertext_len: u64,
}

impl GHash {
    fn new(key: &[u8; 16], data: &[u8]) -> Self {
        let mut ghash = Self {
            function: PolyFunction::new(key),
            data_len: data.len() as u64,
            ciphertext_len: 0,
        };
        ghash.process(data);
        ghash
    }

    fn update(&mut self, input: &[u8]) {
        self.ciphertext_len += input.len() as u64;
        self.process(input);
    }

    fn write_tag(mut self, output: &mut [u8; 16]) {
        BigEndian::write_u64(&mut output[..8], 8 * self.data_len);
        BigEndian::write_u64(&mut output[8..], 8 * self.ciphertext_len);
        self.function.process(output);
        self.function.write_value(output);
    }

    fn process(&mut self, input: &[u8]) {
        for chunk in input.chunks(16) {
            if chunk.len() < 16 {
                let buffer = &mut [0; 16];
                buffer[..chunk.len()].copy_from_slice(chunk);
                self.function.process(buffer);
            } else {
                self.function.process(chunk);
            }
        }
    }
}

#[derive(Clone, Copy)]
struct GFBlock([u64; 2]);

struct PolyFunction {
    key_block: GFBlock,
    state: GFBlock,
}

impl PolyFunction {
    fn new(key: &[u8; 16]) -> Self {
        Self {
            key_block: GFBlock::new(key),
            state: GFBlock([0; 2]),
        }
    }

    fn process(&mut self, input: &[u8]) {
        self.state ^= GFBlock::new(input);
        self.state *= self.key_block;
    }

    fn write_value(self, output: &mut [u8; 16]) {
        BigEndian::write_u64_into(&self.state.0, output);
    }
}

impl GFBlock {
    fn new(bytes: &[u8]) -> Self {
        let mut block = [0; 2];
        BigEndian::read_u64_into(bytes, &mut block);
        GFBlock(block)
    }
}

impl BitXorAssign for GFBlock {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.0.iter_mut().zip(&rhs.0) {
            *l ^= r;
        }
    }
}

impl MulAssign for GFBlock {
    fn mul_assign(&mut self, rhs: Self) {
        let mut z = GFBlock([0; 2]);
        let mut v = rhs;
        for xp in &mut self.0 {
            for _ in 0..64 {
                let mut h = *xp & (1 << 63);
                let mut m = (h as i64 >> 63) as u64;
                for (l, r) in z.0.iter_mut().zip(&v.0) {
                    *l ^= r & m;
                }

                h = v.0[1] << 63;
                m = (h as i64 >> 7) as u64;
                v.0[1] >>= 1;
                v.0[1] |= v.0[0] << 63;
                v.0[0] >>= 1;
                v.0[0] ^= R0 & m;
                *xp <<= 1;
            }
        }
        self.0 = z.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_helpers::*;

    fn check(expected: &str, h: &str, a: &str, c: &str) {
        let h_vec = &h2b(h);
        let h = &mut [0; 16];
        h.copy_from_slice(h_vec);
        let a = &h2b(a);
        let c = &h2b(c);
        let expected = h2b(expected);
        assert_eq!(expected, ghash(h, a, c));
    }

    #[test]
    fn test_case_1_2() {
        let h = "66e94bd4ef8a2c3b884cfa59ca342b2e";
        let a = "";
        let c = "";
        let expected = "00000000000000000000000000000000";
        check(expected, h, a, c);

        let c = "0388dace60b6a392f328c2b971b2fe78";
        let expected = "f38cbb1ad69223dcc3457ae5b6b0f885";
        check(expected, h, a, c);
    }

    #[test]
    fn test_case_15_16_17_18() {
        let h = "acbef20579b4b8ebce889bac8732dad7";
        let a = "";
        let c = "522dc1f099567d07f47f37a32a84427d643a8cdcbfe5c0c97598a2bd2555d1aa\
                 8cb08e48590dbb3da7b08b1056828838c5f61e6393ba7a0abcc9f662898015ad";
        let expected = "4db870d37cb75fcb46097c36230d1612";
        check(expected, h, a, c);

        let a = "feedfacedeadbeeffeedfacedeadbeefabaddad2";
        let c = &c[..120];
        let expected = "8bd0c4d8aacd391e67cca447e8c38f65";
        check(expected, h, a, c);

        let c = "c3762df1ca787d32ae47c13bf19844cbaf1ae14d0b976afac52ff7d79bba9de0\
                 feb582d33934a4f0954cc2363bc73f7862ac430e64abe499f47c9b1f";
        let expected = "75a34288b8c68f811c52b2e9a2f97f63";
        check(expected, h, a, c);

        let c = "5a8def2f0c9e53f1f75d7853659e2a20eeb2b22aafde6419a058ab4f6f746bf4\
                 0fc0c3b780f244452da3ebf1c5d82cdea2418997200ef82e44ae7e3f";
        let expected = "d5ffcf6fc5ac4d69722187421a7f170b";
        check(expected, h, a, c);
    }
}
