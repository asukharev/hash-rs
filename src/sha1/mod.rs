const OUTPUT_LEN: usize = 20;
const R_BYTES: usize = 64;
pub type State = [u32; 5];

pub fn from(m: &[u8]) -> [u8; OUTPUT_LEN] {
    // Pre-processing
    let size = (R_BYTES as f32 * (((m.len() + 8) as f32 / R_BYTES as f32).ceil())) as usize;
    let mut p: Vec<u8> = Vec::with_capacity(size);
    p.extend(m.iter().map(|x| x.clone())); // Copy
    p.extend((0..size - m.len()).map(|_| 0)); // Padding
    p[m.len()] = 0x80; // End of message
    let ptr = p.as_mut_ptr() as *mut u64;
    let word = u64::to_be(m.len() as u64 * 8 as u64);
    unsafe {
        let p = (p.len()/8 - 1) as isize;
        *ptr.offset(p) = word;
    }

    // Initialization
    let mut state: State = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];

    // Main loop
    for pi in p.chunks(R_BYTES) {
        process_block(&mut state, &pi);
    }

    // Produce the final hash value (big-endian) as a 160 bit number:
    let mut digest: [u8; OUTPUT_LEN] = [0; OUTPUT_LEN];
    let output_ptr = digest.as_mut_ptr() as *mut u32;
    for i in 0..state.len() {
        let word = u32::to_be(state[i] as u32);
        unsafe {
            *output_ptr.offset(i as isize) = word;
        }
    }
    digest
}

fn process_block(state: &mut State, block: &[u8]) {
    debug_assert_eq!(block.len(), R_BYTES);

    let mut words = [0u32; 80];
    for (i, chunk) in block.chunks(4).enumerate() {
        words[i] = (chunk[3] as u32) |
                   ((chunk[2] as u32) << 8) |
                   ((chunk[1] as u32) << 16) |
                   ((chunk[0] as u32) << 24);
    }

    fn ff(b: u32, c: u32, d: u32) -> u32 { d ^ (b & (c ^ d)) }
    fn gg(b: u32, c: u32, d: u32) -> u32 { b ^ c ^ d }
    fn hh(b: u32, c: u32, d: u32) -> u32 { (b & c) | (d & (b | c)) }
    fn ii(b: u32, c: u32, d: u32) -> u32 { b ^ c ^ d }

    // Extend the sixteen 32-bit words into eighty 32-bit words
    for i in 16..80 {
        let n = words[i - 3] ^ words[i - 8] ^ words[i - 14] ^ words[i - 16];
        words[i] = n.rotate_left(1);
    }

    // Initialize hash value for this chunk
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];

    // Main loop
    for i in 0..80 {
        let (f, k) = match i {
            0 ... 19 => (ff(b, c, d), 0x5a827999),
            20 ... 39 => (gg(b, c, d), 0x6ed9eba1),
            40 ... 59 => (hh(b, c, d), 0x8f1bbcdc),
            60 ... 79 => (ii(b, c, d), 0xca62c1d6),
            _ => (0, 0),
        };

        let tmp =
            a.rotate_left(5)
            .wrapping_add(f)
            .wrapping_add(e)
            .wrapping_add(k)
            .wrapping_add(words[i]);
        e = d;
        d = c;
        c = b.rotate_left(30);
        b = a;
        a = tmp;
    }

    // Add this chunk's hash to result so far:
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
}
