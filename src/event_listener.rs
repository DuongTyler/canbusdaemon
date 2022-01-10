use enigo::*;
use std::fs::File;
use std::io::Write;

pub fn light_event(data: [u8;8]) {
    println!("[INFO] Recv: Headlight Event [{:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x}]", 
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]);
    let mut brightness_handle = File::open("/sys/class/backlight/rpi_backlight/brightness").unwrap();
    let mut _i: usize = 0;  //doing this quick hack to stop the match from bitching about the return type mismatch (because I'm lazy)
    match data[0] & 0xF0 {
        0x50 | 0x40 => _i = brightness_handle.write(&[0x20]).unwrap(),  //headlights on
        0x00        => _i = brightness_handle.write(&[0xff]).unwrap(),  //headlights off
        _           => println!("[WARN] Event not matched"),
    };
}

pub fn steering_wheel_control_event(data: [u8;8]) {
    println!("[INFO] Recv: Steering Wheel Control Event [{:x}]", data[1]);
    let mut enigo = Enigo::new();
    match data[1] & 0x0F {
        0xA => enigo.key_click(Key::Layout('P')),   //Source
        0xB => enigo.key_click(Key::Layout('N')),   //Next
        0xC => enigo.key_click(Key::Layout('V')),   //Prev
        0xD => enigo.key_click(Key::F8),            //Vol Up
        0xE => enigo.key_click(Key::F7),            //Vol Down
        _   => println!("[WARN] Event not matched"),
    };
}
