use super::{State};

pub trait XOR<T> {
    fn xor_with(&mut self, _: T) -> &mut Self;
}

impl XOR<Vec<u8>> for State {
    fn xor_with(&mut self, s: Vec<u8>) -> &mut Self {
        debug_assert_eq!(s.len() % 8, 0);
        for i in 0..(72 >> 3) {
            self[i] ^= unsafe {
                let ptr = s.as_ptr() as *const u64;
                *(ptr.offset(i as isize))
            };
        }
        self
    }
}
