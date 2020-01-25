/// Calculate the CRC8 checksum.
///
/// Implementation based on the reference implementation by Sensirion.
pub(crate) fn crc8(data: &[u8]) -> u8 {
    const CRC8_POLYNOMIAL: u8 = 0x31;
    let mut crc: u8 = 0xff;
    for byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if (crc & 0x80) > 0 {
                crc = (crc << 1) ^ CRC8_POLYNOMIAL;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the crc8 function against the test value provided in the
    /// SHTC3 datasheet (section 5.10).
    #[test]
    fn crc8_test_value() {
        assert_eq!(crc8(&[0x00]), 0xac);
        assert_eq!(crc8(&[0xbe, 0xef]), 0x92);
    }
}
