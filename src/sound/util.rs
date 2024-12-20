pub struct LittleEndian;

impl LittleEndian {
    pub fn read_u16(data: &[u8]) -> u16 {
        ((data[1] as u16) << 8) | data[0] as u16
    }

    pub fn read_u32(data: &[u8]) -> u32 {
        ((data[3] as u32) << 24) | ((data[2] as u32) << 16) | 
        ((data[1] as u32) << 8) | data[0] as u32
    }
 }

mod tests {
    #![allow(unused)]
    use super::*;

    #[test]
    fn test_little_endian_u16() {
        assert_eq!(LittleEndian::read_u16(&[0x0A, 0x00]), 0x000A);
        assert_eq!(LittleEndian::read_u16(&[0x0D, 0x0F]), 0x0F0D);
        assert_eq!(LittleEndian::read_u16(&[0xAB, 0x52]), 0x52AB);
    }

    #[test]
    fn test_little_endian_u32() {
        assert_eq!(LittleEndian::read_u32(&[0x12, 0x34, 0x56, 0x78]), 0x78563412);
        assert_eq!(LittleEndian::read_u32(&[0x0A, 0x00, 0x00, 0x00]), 0x0000000A);
    }
}