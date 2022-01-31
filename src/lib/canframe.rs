use std::mem;
use std::convert::TryInto;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_populate() {
        let canframe = populate_canframe(&[u8::from(0xEF), 0xBE, 0xAD, 0xDE,       //Magic
                                            0xA0, 0xB0, 0xC0, 0xD0,     //id
                                            0x32, 0x00, 0x00, 0x00,     //frame type
                                            0x08, 0x00, 0x00, 0x00,     //len
                                            0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x1A, 0x1B  //data
                                            ]);
        assert_eq!(canframe.magic, 0xDEADBEEF);
        assert_eq!(canframe.id, 0xD0C0B0A0);
        assert_eq!(canframe.frame_type, 0x32);
        assert_eq!(canframe.len, 0x8);
    }

    #[test]
    #[should_panic]
    fn test_populate_panic_bad_magic () {
        populate_canframe(&[u8::from(0xFF), 0xBE, 0xAD, 0xDE,       //Bad Magic
                                            0xA0, 0xB0, 0xC0, 0xD0,     //id
                                            0x32, 0x00, 0x00, 0x00,     //frame type
                                            0x08, 0x00, 0x00, 0x00,     //len
                                            0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x1A, 0x1B  //data
                                            ]);
    }
}

pub struct CanFrame {
    pub magic: u32,
    pub id: u32,
    pub frame_type: u32,
    pub len: u32,
    pub data: [u8; 8],
}

pub fn populate_canframe(buf: &[u8;mem::size_of::<CanFrame>()]) -> CanFrame {
    let mut frame: CanFrame = CanFrame {
        magic: 0x0,
        id: 0x0,
        frame_type: 0x0,
        len: 0x0,
        data: [0;8],
    };
    //data is read as little endian
    let magic = &buf[0..4].try_into().expect("incorrect magic len");
    frame.magic = u32::from_le_bytes(*magic);
    if frame.magic != 0xDEADBEEF {
        println!("Magic Incorrect: {:x}", frame.magic);
        for i in buf {
            print!("{:x}", i);
        }
        panic!("Magic is Incorrect. Cannot correct.");
    }
    let id = &buf[4..8].try_into().expect("incorrect id len");
    frame.id = u32::from_le_bytes(*id);
    let frame_type = &buf[8..12].try_into().expect("incorrect type len");
    frame.frame_type = u32::from_le_bytes(*frame_type);
    let len = &buf[12..16].try_into().expect("incorrect type len");
    frame.len = u32::from_le_bytes(*len);
    let data = &buf[16..24].try_into().expect("wrong data size");
    frame.data = *data;
    return frame;
}
