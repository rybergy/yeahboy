pub fn unpack_u16(bytes: u16) -> (u8, u8) {
    return ((bytes >> 8) as u8, bytes as u8);
}

pub fn pack_u16(lower: u8, upper: u8) -> u16 {
    ((lower as u16) << 8) | (upper as u16)
}
