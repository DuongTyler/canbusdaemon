use enigo::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn light_event(data: [u8;8]) {
    println!("[INFO] Recv: Headlight Event [{:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x}]",
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]);
    if !Path::new("/sys/class/backlight/rpi_backlight/brightness").exists() {
        panic!("[ERRO] Missing backlight control path /sys/class/backlight/rpi_backlight/brightness");
    }
    let mut brightness_handle = File::open("/sys/class/backlight/rpi_backlight/brightness").unwrap();
    let mut _i: usize = 0;  //doing this quick hack to stop the match from bitching about the return type mismatch (because I'm lazy)
    match data[0] & 0xF0 {
        0x50 | 0x40 => _i = brightness_handle.write(&[0x20]).unwrap(),  //headlights on, brightness down
        0x00        => _i = brightness_handle.write(&[0xff]).unwrap(),  //headlights off, brightness up
        _           => println!("[WARN] Event not matched"),
    };
}

pub fn steering_wheel_control_event(data: [u8;8]) {
    println!("[INFO] Recv: Steering Wheel Control Event [{:x}]", data[1]);
    let mut _enigo = Enigo::new();
    #[cfg(not(test))]
    match data[1] & 0x0F {
        0xA => _enigo.key_click(Key::Layout('P')),   //Source
        0xB => _enigo.key_click(Key::Layout('N')),   //Next
        0xC => _enigo.key_click(Key::Layout('V')),   //Prev
        0xD => _enigo.key_click(Key::F8),            //Vol Up
        0xE => _enigo.key_click(Key::F7),            //Vol Down
        _   => println!("[WARN] Event not matched"),
    };
}
