use std::fmt;
mod sha1;
mod sha3;

pub struct Digest(Vec<u8>);

impl Digest {
    pub fn sha1(m: &[u8]) -> Digest {
        let a = sha1::from(m);
        let mut digest: Vec<u8> = Vec::new();
        digest.extend(a.iter().cloned());
        Digest(digest)
    }

    pub fn sha3(m: &[u8]) -> Digest {
        let a = sha3::from(m);
        let mut digest: Vec<u8> = Vec::new();
        digest.extend(a.iter().cloned());
        Digest(digest)
    }
}

impl Clone for Digest {
    fn clone(&self) -> Self {
        Digest(self.0.clone())
    }
}

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
