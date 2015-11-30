// http://keccak.noekeon.org/Keccak-implementation-3.2.pdf
mod consts;
mod ext;
use keccak::ext::XOR;
use super::{Digest, OUTPUT_LEN};

pub type State = [u64; 25];

const LANES: usize = 25;
const R_BYTES: usize = (LANES * 8) - (2 * OUTPUT_LEN);

pub fn from(m: &[u8]) -> Digest {
    // Padding
    let mut size = R_BYTES * (m.len() as f64 / R_BYTES as f64).ceil() as usize;
    if size == m.len() {
        size += R_BYTES;
    }
    let mut p: Vec<u8> = Vec::with_capacity(size);
    p.extend(m.iter().map(|x| x.clone()));
    p.extend((0..size - m.len()).map(|_| 0));
    p[m.len()] = 1;
    p[size - 1] ^= 0x80;

    // Initialization
    let mut state: State = [0; 25];

    // Absorbing
    for pi in p.chunks(R_BYTES) {
        state.xor_with(pi.to_vec());
        keccak_f(&mut state);
    }

    // Squeezing
    let mut digest: [u8; OUTPUT_LEN] = [0; 64];
    let output_ptr = digest.as_mut_ptr() as *mut u64;
    for i in 0..(OUTPUT_LEN / 8) {
        let word = u64::to_le(state[i]);
        unsafe {
            *output_ptr.offset(i as isize) = word;
        }
    }

    Digest(digest)
}

pub fn keccak_f(a: &mut State) {
    for i in 0..consts::ROUNDS {
        round_f(a, i);
    }
}

pub fn round_f(a: &mut State, round: usize) {
    let mut c: [u64; 5] = [0; 5];
    let mut d: [u64; 5] = [0; 5];
    let mut b: [[u64; 5]; 5] = [[0; 5]; 5];

    { // step θ
        for i in 0..5 {
          c[i] = a[i] ^ a[i+5] ^ a[i+10] ^ a[i+15] ^ a[i+20];
        }
        for i in 0..5 {
          d[i] = c[(i + 4) % 5] ^ c[(i + 1) % 5].rotate_left(1);
        }
        for i in 0..5 {
          for j in 0..5 {
            a[i + 5*j] ^= d[i];
          }
        }
    }

    { // step ρ и π
        for i in 0..5 {
          for j in 0..5 {
            b[j][(2*i + 3*j) % 5] = a[i + 5*j].rotate_left(consts::R[i][j] as u32);
          }
        }
    }

    { // step χ
        for i in 0..5 {
          for j in 0..5 {
            a[i + 5*j] = b[i][j] ^ ((!b[(i + 1) % 5][j]) & b[(i + 2) % 5][j]);
          }
        }
    }

    // step ι
    a[0] ^= consts::ROUND_CONSTANTS[round];
}
