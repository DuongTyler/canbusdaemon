use enigo::*;

pub fn headlight_event(data: [u8;8]) {
    let mut enigo = Enigo::new();
    println!("Headlight Event");
    enigo.key_click(Key::Layout('a'));
}

pub fn steering_wheel_control_event(data: [u8;8]) {
    println!("Steering Wheel Event");
}
