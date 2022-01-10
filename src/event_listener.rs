use enigo::*;
use std::fs::File;
use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn light_event_test() {
        #[cfg(target_arch = "arm")] //welp, shitty test, but there's not much to test here ngl. It doesn't work on non-raspberry pis
        assert_eq!(light_event([0x50, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]), 0x5);
    }
    #[test]
    fn steering_wheel_control_event_test() {
        assert_eq!(steering_wheel_control_event([0x0, 0x0A, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]), 0xA);
        assert_eq!(steering_wheel_control_event([0x0, 0x0B, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]), 0xB);
        assert_eq!(steering_wheel_control_event([0x0, 0x0C, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]), 0xC);
        assert_eq!(steering_wheel_control_event([0x0, 0x0D, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]), 0xD);
        assert_eq!(steering_wheel_control_event([0x0, 0x0E, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]), 0xE);
    }
}

pub fn light_event(data: [u8;8]) -> u8 {
    println!("[INFO] Recv: Headlight Event [{:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x}]",
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]);
    let mut brightness_handle = File::open("/sys/class/backlight/rpi_backlight/brightness").unwrap();
    let mut _i: usize = 0;  //doing this quick hack to stop the match from bitching about the return type mismatch (because I'm lazy)
    let input = data[0] & 0xF0;
    match input {
        0x50 | 0x40 => _i = brightness_handle.write(&[0x20]).unwrap(),  //headlights on
        0x00        => _i = brightness_handle.write(&[0xff]).unwrap(),  //headlights off
        _           => println!("[WARN] Event not matched"),
    };
    return input;
}

pub fn steering_wheel_control_event(data: [u8;8]) -> u8 {
    println!("[INFO] Recv: Steering Wheel Control Event [{:x}]", data[1]);
    let mut _enigo = Enigo::new();
    let input = data[1] & 0x0F;
    #[cfg(target_arch = "arm")]
    match input {
        0xA => _enigo.key_click(Key::Layout('P')),   //Source
        0xB => _enigo.key_click(Key::Layout('N')),   //Next
        0xC => _enigo.key_click(Key::Layout('V')),   //Prev
        0xD => _enigo.key_click(Key::F8),            //Vol Up
        0xE => _enigo.key_click(Key::F7),            //Vol Down
        _   => println!("[WARN] Event not matched"),
    };
    return input;
}
