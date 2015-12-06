use super::{Digest, OUTPUT_LEN};

pub fn from(m: &[u8]) -> Digest {
    let mut digest: [u8; OUTPUT_LEN] = [0; 64];
    Digest(digest)
}
