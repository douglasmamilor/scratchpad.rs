pub struct BitMapHeader {
    pub file_type: [u8; 2],
    pub file_size: u32,
    pub reserved1: u16,
    pub reserved2: u16,
    pub offset: u32,
}

pub fn read_bitmap(b: &[u8]) {
    let header = &b[0..14];

    let bitmap_header = BitMapHeader {
        file_type: [header[0], header[1]],
        file_size: u32::from_le_bytes([header[2], header[3], header[4], header[5]]),
        reserved1: u16::from_le_bytes([header[6], header[7]]),
        reserved2: u16::from_le_bytes([header[8], header[9]]),
        offset: u32::from_le_bytes([header[10], header[11], header[12], header[13]]),
    };

    assert_eq!(bitmap_header.file_type, [0x42, 0x4D]); // 'BM'
}
