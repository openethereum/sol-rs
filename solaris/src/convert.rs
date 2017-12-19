// TODO [snd] remove this once ethabi is modified
// and can automatically convert str into bytes32
pub fn bytes32(s: &str) -> [u8; 32] {
    let bytes = s.as_bytes();
    let mut ret = [0u8; 32];
    let s = 32 - bytes.len();
    ret[s..].copy_from_slice(bytes);
    ret
}
