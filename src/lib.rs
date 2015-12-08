use std::fmt;

// mod keccak;
mod sha1;

const OUTPUT_LEN: usize = 20;
// const OUTPUT_LEN: usize = 64;

pub struct Digest(pub [u8; OUTPUT_LEN]);

impl Digest {
    // pub fn keccak(m: &[u8]) -> Digest {
    //     keccak::from(m)
    // }

    pub fn sha1(m: &[u8]) -> Digest {
        sha1::from(m)
    }
}

impl Clone for Digest {
    fn clone(&self) -> Self { *self }
}

impl Copy for Digest {}

impl fmt::LowerHex for Digest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0.iter() {
            try!(write!(f, "{:02x}", byte));
        }
        Ok(())
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl fmt::Debug for Digest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl PartialEq for Digest {
    fn eq(&self, other: &Self) -> bool {
        // yay, timing attack
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            if a != b {
                return false;
            }
        }
        true
    }
}

impl Eq for Digest {}
