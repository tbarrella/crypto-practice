use std::iter;
use byteorder::{BigEndian, ByteOrder};

pub const SHA512_OUTPUT_LEN: usize = 64;
pub const SHA384_OUTPUT_LEN: usize = 48;

pub fn sha512(msg: &[u8]) -> [u8; SHA512_OUTPUT_LEN] {
    let mut sha = Sha::new(SHA512);
    let mut digest = [0; SHA512_OUTPUT_LEN];
    sha.process(msg);
    sha.write_digest_into(&mut digest);
    digest
}

pub fn sha384(msg: &[u8]) -> [u8; SHA384_OUTPUT_LEN] {
    let mut sha = Sha::new(SHA384);
    let mut digest = [0; SHA384_OUTPUT_LEN];
    sha.process(msg);
    sha.write_digest_into(&mut digest);
    digest
}

struct Hash {
    output_len: usize,
    initial_state: &'static [u64],
}

static SHA512: &'static Hash = &Hash {
    output_len: SHA512_OUTPUT_LEN,
    initial_state: &[
        0x6a09_e667_f3bc_c908,
        0xbb67_ae85_84ca_a73b,
        0x3c6e_f372_fe94_f82b,
        0xa54f_f53a_5f1d_36f1,
        0x510e_527f_ade6_82d1,
        0x9b05_688c_2b3e_6c1f,
        0x1f83_d9ab_fb41_bd6b,
        0x5be0_cd19_137e_2179,
    ],
};

static SHA384: &'static Hash = &Hash {
    output_len: SHA384_OUTPUT_LEN,
    initial_state: &[
        0xcbbb_9d5d_c105_9ed8,
        0x629a_292a_367c_d507,
        0x9159_015a_3070_dd17,
        0x152f_ecd8_f70e_5939,
        0x6733_2667_ffc0_0b31,
        0x8eb4_4a87_6858_1511,
        0xdb0c_2e0d_64f9_8fa7,
        0x47b5_481d_befa_4fa4,
    ],
};

const K: [u64; 80] = [
    0x428a_2f98_d728_ae22,
    0x7137_4491_23ef_65cd,
    0xb5c0_fbcf_ec4d_3b2f,
    0xe9b5_dba5_8189_dbbc,
    0x3956_c25b_f348_b538,
    0x59f1_11f1_b605_d019,
    0x923f_82a4_af19_4f9b,
    0xab1c_5ed5_da6d_8118,
    0xd807_aa98_a303_0242,
    0x1283_5b01_4570_6fbe,
    0x2431_85be_4ee4_b28c,
    0x550c_7dc3_d5ff_b4e2,
    0x72be_5d74_f27b_896f,
    0x80de_b1fe_3b16_96b1,
    0x9bdc_06a7_25c7_1235,
    0xc19b_f174_cf69_2694,
    0xe49b_69c1_9ef1_4ad2,
    0xefbe_4786_384f_25e3,
    0x0fc1_9dc6_8b8c_d5b5,
    0x240c_a1cc_77ac_9c65,
    0x2de9_2c6f_592b_0275,
    0x4a74_84aa_6ea6_e483,
    0x5cb0_a9dc_bd41_fbd4,
    0x76f9_88da_8311_53b5,
    0x983e_5152_ee66_dfab,
    0xa831_c66d_2db4_3210,
    0xb003_27c8_98fb_213f,
    0xbf59_7fc7_beef_0ee4,
    0xc6e0_0bf3_3da8_8fc2,
    0xd5a7_9147_930a_a725,
    0x06ca_6351_e003_826f,
    0x1429_2967_0a0e_6e70,
    0x27b7_0a85_46d2_2ffc,
    0x2e1b_2138_5c26_c926,
    0x4d2c_6dfc_5ac4_2aed,
    0x5338_0d13_9d95_b3df,
    0x650a_7354_8baf_63de,
    0x766a_0abb_3c77_b2a8,
    0x81c2_c92e_47ed_aee6,
    0x9272_2c85_1482_353b,
    0xa2bf_e8a1_4cf1_0364,
    0xa81a_664b_bc42_3001,
    0xc24b_8b70_d0f8_9791,
    0xc76c_51a3_0654_be30,
    0xd192_e819_d6ef_5218,
    0xd699_0624_5565_a910,
    0xf40e_3585_5771_202a,
    0x106a_a070_32bb_d1b8,
    0x19a4_c116_b8d2_d0c8,
    0x1e37_6c08_5141_ab53,
    0x2748_774c_df8e_eb99,
    0x34b0_bcb5_e19b_48a8,
    0x391c_0cb3_c5c9_5a63,
    0x4ed8_aa4a_e341_8acb,
    0x5b9c_ca4f_7763_e373,
    0x682e_6ff3_d6b2_b8a3,
    0x748f_82ee_5def_b2fc,
    0x78a5_636f_4317_2f60,
    0x84c8_7814_a1f0_ab72,
    0x8cc7_0208_1a64_39ec,
    0x90be_fffa_2363_1e28,
    0xa450_6ceb_de82_bde9,
    0xbef9_a3f7_b2c6_7915,
    0xc671_78f2_e372_532b,
    0xca27_3ece_ea26_619c,
    0xd186_b8c7_21c0_c207,
    0xeada_7dd6_cde0_eb1e,
    0xf57d_4f7f_ee6e_d178,
    0x06f0_67aa_7217_6fba,
    0x0a63_7dc5_a2c8_98a6,
    0x113f_9804_bef9_0dae,
    0x1b71_0b35_131c_471b,
    0x28db_77f5_2304_7d84,
    0x32ca_ab7b_40c7_2493,
    0x3c9e_be0a_15c9_bebc,
    0x431d_67c4_9c10_0d4c,
    0x4cc5_d4be_cb3e_42b6,
    0x597f_299c_fc65_7e2a,
    0x5fcb_6fab_3ad6_faec,
    0x6c44_198c_4a47_5817,
];

struct Sha {
    state: [u64; 8],
    output_len: usize,
}

impl Sha {
    fn new(hash: &'static Hash) -> Self {
        let mut sha = Self {
            state: [0; 8],
            output_len: hash.output_len,
        };
        sha.state.copy_from_slice(hash.initial_state);
        sha
    }

    fn process(&mut self, message: &[u8]) {
        let mut message = message.to_vec();
        Self::pad(&mut message);
        let mut w = [0; 80];
        for chunk in message.chunks(128) {
            BigEndian::read_u64_into(chunk, &mut w[..16]);
            for t in 16..80 {
                w[t] = Self::ssig1(w[t - 2])
                    .wrapping_add(w[t - 7])
                    .wrapping_add(Self::ssig0(w[t - 15]))
                    .wrapping_add(w[t - 16]);
            }
            let mut a = self.state[0];
            let mut b = self.state[1];
            let mut c = self.state[2];
            let mut d = self.state[3];
            let mut e = self.state[4];
            let mut f = self.state[5];
            let mut g = self.state[6];
            let mut h = self.state[7];
            for (&kt, &wt) in K.iter().zip(w.iter()) {
                let t1 = h.wrapping_add(Self::bsig1(e))
                    .wrapping_add(Self::ch(e, f, g))
                    .wrapping_add(kt)
                    .wrapping_add(wt);
                let t2 = Self::bsig0(a).wrapping_add(Self::maj(a, b, c));
                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(t1);
                d = c;
                c = b;
                b = a;
                a = t1.wrapping_add(t2);
            }
            self.state[0] = self.state[0].wrapping_add(a);
            self.state[1] = self.state[1].wrapping_add(b);
            self.state[2] = self.state[2].wrapping_add(c);
            self.state[3] = self.state[3].wrapping_add(d);
            self.state[4] = self.state[4].wrapping_add(e);
            self.state[5] = self.state[5].wrapping_add(f);
            self.state[6] = self.state[6].wrapping_add(g);
            self.state[7] = self.state[7].wrapping_add(h);
        }
    }

    fn write_digest_into(&self, buf: &mut [u8]) {
        assert_eq!(self.output_len, buf.len());
        BigEndian::write_u64_into(&self.state[..self.output_len / 8], buf);
    }

    fn ch(x: u64, y: u64, z: u64) -> u64 {
        (x & y) ^ (!x & z)
    }

    fn maj(x: u64, y: u64, z: u64) -> u64 {
        (x & y) ^ (x & z) ^ (y & z)
    }

    fn bsig0(x: u64) -> u64 {
        x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39)
    }

    fn bsig1(x: u64) -> u64 {
        x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41)
    }

    fn ssig0(x: u64) -> u64 {
        x.rotate_right(1) ^ x.rotate_right(8) ^ (x >> 7)
    }

    fn ssig1(x: u64) -> u64 {
        x.rotate_right(19) ^ x.rotate_right(61) ^ (x >> 6)
    }

    /// Only supports messages with at most 2^64 - 1 bits for now
    fn pad(bytes: &mut Vec<u8>) {
        let len = len(bytes);
        bytes.push(0x80);
        let padding = (128 + 112 - bytes.len() % 128) % 128;
        bytes.extend(iter::repeat(0).take(padding));
        bytes.extend_from_slice(&[0; 8]);
        bytes.extend_from_slice(&len);
    }
}

fn len(bytes: &[u8]) -> [u8; 8] {
    let mut len = [0; 8];
    BigEndian::write_u64(&mut len, 8 * bytes.len() as u64);
    len
}

#[cfg(test)]
mod tests {
    use sha::*;
    use test_helpers::*;

    const TEST1: &[u8] = b"abc";
    const TEST2: &[u8] = b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmn\
        hijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu";
    const TEST3: &[u8] = &[0x61; 1000000];

    #[test]
    fn test_pad() {
        let mut message = vec![0b01100001, 0b01100010, 0b01100011, 0b01100100, 0b01100101];
        let expected = h2b(
            "6162636465800000000000000000000000000000000000000000000000000000\
             0000000000000000000000000000000000000000000000000000000000000000\
             0000000000000000000000000000000000000000000000000000000000000000\
             0000000000000000000000000000000000000000000000000000000000000028",
        );
        Sha::pad(&mut message);
        assert_eq!(expected, message);
    }

    fn check(exp512: &str, exp384: &str, message: &[u8]) {
        let actual = sha512(message);
        assert_eq!(h2b(exp512), actual.to_vec());

        let actual = sha384(message);
        assert_eq!(h2b(exp384), actual.to_vec());
    }

    #[test]
    fn test_digest() {
        let mut exp512 = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce\
                          47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e";
        let mut exp384 = "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da\
                          274edebfe76f65fbd51ad2f14898b95b";
        check(exp512, exp384, &[]);

        exp512 = "DDAF35A193617ABACC417349AE20413112E6FA4E89A97EA20A9EEEE64B55D39A\
                  2192992A274FC1A836BA3C23A3FEEBBD454D4423643CE80E2A9AC94FA54CA49F";
        exp384 = "CB00753F45A35E8BB5A03D699AC65007272C32AB0EDED1631A8B605A43FF5BED\
                  8086072BA1E7CC2358BAECA134C825A7";
        check(exp512, exp384, TEST1);

        exp512 = "8E959B75DAE313DA8CF4F72814FC143F8F7779C6EB9F7FA17299AEADB6889018\
                  501D289E4900F7E4331B99DEC4B5433AC7D329EEB6DD26545E96E55B874BE909";
        exp384 = "09330C33F71147E83D192FC782CD1B4753111B173B3B05D22FA08086E3B0F712\
                  FCC7C71A557E2DB966C3E9FA91746039";
        check(exp512, exp384, TEST2);

        exp512 = "E718483D0CE769644E2E42C7BC15B4638E1F98B13B2044285632A803AFA973EB\
                  DE0FF244877EA60A4CB0432CE577C31BEB009C5C2C49AA2E4EADB217AD8CC09B";
        exp384 = "9D0E1809716474CB086E834E310A4A1CED149E9C00F248527972CEC5704C2A5B\
                  07B8B3DC38ECC4EBAE97DDD87F3D8985";
        check(exp512, exp384, TEST3);

        let mut test4 = String::new();
        for _ in 0..80 {
            test4.push_str("01234567");
        }
        exp512 = "89D05BA632C699C31231DED4FFC127D5A894DAD412C0E024DB872D1ABD2BA814\
                  1A0F85072A9BE1E2AA04CF33C765CB510813A39CD5A84C4ACAA64D3F3FB7BAE9";
        exp384 = "2FC64A4F500DDB6828F6A3430B8DD72A368EB7F3A8322A70BC84275B9C0B3AB0\
                  0D27A5CC3C2D224AA6B61A0D79FB4596";
        check(exp512, exp384, test4.as_bytes());
    }
}
