use enigo::*;
use std::fs::File;
use std::io::Write;

pub fn headlight_event(data: [u8;8]) {
    println!("[INFO] Recv: Headlight Event");
    let mut brightness_handle = File::open("/sys/class/backlight/rpi_backlight/brightness").unwrap();
    let mut _i: usize = 0;
    match data[0] & 0xF0 {
        0x50 | 0x40 => _i = brightness_handle.write(&[0x20]).unwrap(),
        0x00        => _i = brightness_handle.write(&[0xff]).unwrap(),
        _           => println!("[WARN] Event not matched"),
    };
}

pub fn steering_wheel_control_event(data: [u8;8]) {
    println!("[INFO] Recv: Steering Wheel Control Event [{:x}]", data[1]);
    let mut enigo = Enigo::new();
    match data[1] & 0x0F {
        0xA => enigo.key_click(Key::Layout('P')),
        0xB => enigo.key_click(Key::Layout('N')),
        0xC => enigo.key_click(Key::Layout('V')),
        0xD => enigo.key_click(Key::F8),
        0xE => enigo.key_click(Key::F7),
        _   => println!("[WARN] Event not matched"),
    };
}
