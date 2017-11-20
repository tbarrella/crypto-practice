use sha2::{HashFunction, Sha384};

const MAX_DIGEST_SIZE: usize = 64;
const IPAD: u8 = 0x36;
const OPAD: u8 = 0x5c;

pub struct Hmac<T> {
    inner_hash_function: T,
    outer_hash_function: T,
}

pub fn hmac_sha384(key: &[u8], message: &[u8]) -> [u8; Sha384::DIGEST_SIZE] {
    let mut digest = [0; Sha384::DIGEST_SIZE];
    let mut hmac = Hmac::<Sha384>::new(key);
    hmac.update(message);
    hmac.write_digest(&mut digest);
    digest
}

impl<T: HashFunction> Hmac<T> {
    pub fn new(key: &[u8]) -> Self {
        let mut hashed_key;
        let new_key;
        if key.len() > T::BLOCK_SIZE {
            hashed_key = [0; MAX_DIGEST_SIZE];
            let mut hash_function = T::default();
            hash_function.update(key);
            hash_function.write_digest(&mut hashed_key[..T::DIGEST_SIZE]);
            new_key = &hashed_key[..T::DIGEST_SIZE];
        } else {
            new_key = key;
        }

        let mut hmac = Self {
            inner_hash_function: T::default(),
            outer_hash_function: T::default(),
        };
        for byte in new_key {
            hmac.inner_hash_function.update(&[byte ^ IPAD]);
            hmac.outer_hash_function.update(&[byte ^ OPAD]);
        }
        for _ in new_key.len()..T::BLOCK_SIZE {
            hmac.inner_hash_function.update(&[IPAD]);
            hmac.outer_hash_function.update(&[OPAD]);
        }
        hmac
    }

    pub fn update(&mut self, input: &[u8]) {
        self.inner_hash_function.update(input);
    }

    pub fn write_digest(&mut self, output: &mut [u8]) {
        self.inner_hash_function.write_digest(output);
        self.outer_hash_function.update(output);
        self.outer_hash_function.write_digest(output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_helpers::*;

    fn check(expected: &str, key: &[u8], data: &[u8]) {
        let expected = h2b(expected);
        let mut actual = hmac_sha384(key, data);
        assert_eq!(expected, actual.to_vec());

        let mut hmac = Hmac::<Sha384>::new(key);
        for word in data.chunks(4) {
            hmac.update(word);
        }
        hmac.write_digest(&mut actual);
        assert_eq!(expected, actual.to_vec());
    }

    #[test]
    fn test_digest() {
        let key = h2b("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b");
        let data = b"Hi There";
        let mut expected = "afd03944d84895626b0825f4ab46907f15f9dadbe4101ec682aa034c7cebc59c\
                            faea9ea9076ede7f4af152e8b2fa9cb6";
        check(expected, &key, data);

        let key = b"Jefe";
        let data = b"what do ya want for nothing?";
        expected = "af45d2e376484031617f78d2b58a6b1b9c7ef464f5a01b47e42ec3736322445e\
                    8e2240ca5e69e2c78b3239ecfab21649";
        check(expected, key, data);

        let key = [0xaa; 20];
        let mut data = [0xdd; 50];
        expected = "88062608d3e6ad8a0aa2ace014c8a86f0aa635d947ac9febe83ef4e55966144b\
                    2a5ab39dc13814b94e3ab6e101a34f27";
        check(expected, &key, &data);

        let key: Vec<_> = (0x01..0x1a).collect();
        data = [0xcd; 50];
        expected = "3e8a69b7783c25851933ab6290af6ca77a9981480850009cc5577c6e1f573b4e\
                    6801dd23c4a7d679ccf8a386c674cffb";
        check(expected, &key, &data);

        let key = [0xaa; 131];
        let data = b"Test Using Larger Than Block-Size Key - Hash Key First";
        expected = "4ece084485813e9088d2c63a041bc5b44f9ef1012a2b588f3cd11f05033ac4c6\
                    0c2ef6ab4030fe8296248df163f44952";
        check(expected, &key, data);

        let data =
            b"This is a test using a larger than block-size key and a larger than block-\
              size data. The key needs to be hashed before being used by the HMAC algorithm.";
        expected = "6617178e941f020d351e2f254e8fd32c602420feb0b8fb9adccebb82461e99c5\
                    a678cc31e799176d3860e6110c46523e";
        check(expected, &key, data);
    }
}
