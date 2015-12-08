#![allow(unused_variables, dead_code)]

extern crate hash;

fn check_224(input: &str, output: &str) {
}

fn check_256(input: &str, output: &str) {
}

fn check_384(input: &str, output: &str) {
}

fn check_512(input: &str, output: &str) {
    let digest = hash::Digest::sha3(&input.as_bytes());
    assert_eq!(format!("{}", digest), output);
}

#[test]
fn known_512_digests() {
    check_512("", "0eab42de4c3ceb9235fc91acffe746b29c29a8c366b7c60e4e67c466f36a4304c00fa9caf9d87976ba469bcbe06713b435f091ef2769fb160cdab33d3670680e");
    check_512("A", "421a35a60054e5f383b6137e43d44e998f496748cc77258240ccfaa8730b51f40cf47c1bc09c728a8cd4f096731298d51463f15af89543fed478053346260c38");

    let block = "012345678901234567890123456789012345678901234567890123456789012345678901";
    // assert_eq!(block.len(), keccak::R_BYTES);
    check_512(&block[..69], "5308edb15b386c77921367c483b65c7d3fe23c4b423ddb8df4a5b7f0de40b0ca60b3de5dbb8b153252bd1e66cdd10c1009cdd2ceb23b61bfc44f8ca4209aa75a");
    check_512(&block[..70], "c5eba2e8c8fe3a045d3de364a4581f65ad9e54756b58b957364304d209ff10783e58c88075efa3d92cdfa2c243247d8ff7ea360495632b023fa06cfabbc9d30a");
    check_512(&block[..71], "3173e7abc754a0b2909410d78986428a9183e996864af02f421d273d9fa1b4e4a5b14e2998b20767712f53a01ff8f6ae2c3e71e51e2c0f24257b03e6da09eb77");
    check_512(block, "90b1d032c3bf06dcc78a46fe52054bab1250600224bfc6dfbfb40a7877c55e89bb982799a2edf198568a4166f6736678b45e76b12fac813cfdf0a76714e5eae8");
    check_512(block, "90b1d032c3bf06dcc78a46fe52054bab1250600224bfc6dfbfb40a7877c55e89bb982799a2edf198568a4166f6736678b45e76b12fac813cfdf0a76714e5eae8");
    check_512(&[block, "2"].concat(), "7ecc23723c40dc1154611e2ba1752a5cb6082f592a10b8e3f3817ea634e40d272f2ecf72a99374860c311b8cb6cdadcc862198ac394c7f49a36687fb99f93501");
    check_512(&[block, block].concat(), "bad62fb72bc1d1ebc117523791dd49a03a65ffd3805363e902378256d34f1d4a6c6afdad5aeaea3bfc1a92fd10c3d97d8ad6b5df85e5a0cd7eb43770356dfcc2");
    check_512(&[block, block, block].concat(), "d22e9b6978a012bcb8a6a6e44c919336d8e847994190dbdf839ba10d8fc9c231a33bab45e90b2ceaa60d117331b617309c6f9d07c7bc2aa0a54c1d4622d6388d");
}

fn check_sha1(input: &str, output: &str) {
    let digest = hash::Digest::sha1(input.as_bytes());
    assert_eq!(format!("{}", digest), output);
}

#[test]
fn known_sha1_digests() {
    check_sha1("The quick brown fox jumps over the lazy dog", "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
    check_sha1("The quick brown fox jumps over the lazy cog", "de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3");
    check_sha1("", "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    check_sha1("testing\n", "9801739daae44ec5293d4e1f53d3f4d2d426d91c");
    check_sha1("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "025ecbd5d70f8fb3c5457cd96bab13fda305dc59");
    check_sha1("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", "956a4ea9812940d46745e590ae00897d20c7ad0a");
}
