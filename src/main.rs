use serialport;
use std::mem;
use std::time::Duration;
use std::convert::TryInto;
use std::collections::HashMap;

mod eventListener;

struct CanFrame {
    magic: u32,
    id: u32,
    frame_type: u32,
    len: u32,
    data: [u8; 8],
}

fn populate_canframe(buf: &[u8;mem::size_of::<CanFrame>()]) -> CanFrame {
    let mut frame: CanFrame = CanFrame {
        magic: 0x0,
        id: u32::MAX,
        frame_type: u32::MAX,
        len: 0x0,
        data: [0;8],
    };
    //data is read as little endian
    let magic = &buf[0..4].try_into().expect("incorrect magic len");
    frame.magic = u32::from_le_bytes(*magic);
    if frame.magic != 0xDEADBEEF {
        println!("Magic Incorrect: {:x}", frame.magic);
        for i in buf {
            print!("{:x}", i)
        }
        panic!("Magic is Incorrect. Cannot correct.");
    }
    let id = &buf[4..8].try_into().expect("incorrect id len");
    frame.id = u32::from_le_bytes(*id);
    let len = &buf[8..12].try_into().expect("incorrect type len");
    frame.len = u32::from_le_bytes(*len);
    let frame_type = &buf[12..16].try_into().expect("incorrect type len");
    frame.frame_type = u32::from_le_bytes(*frame_type);
    let data = &buf[16..24].try_into().expect("wrong data size");
    frame.data = *data;
    return frame;
}

fn main() {
    eventListener::headlight_event([1,1,1,1,1,1,1,1]);
    let sensor_listeners = HashMap::from([
        (0x0AF87010, eventListener::headlight_event),
        //(0x12F95757, eventListener::steering_wheel_control_event),
    ]);


    let ports = serialport::available_ports().unwrap();
    let mut port_name = String::new();
    for i in ports.iter() {
        println!("{:?}",i);
        port_name = i.port_name.clone();
    }
    println!("reading from {}", port_name);
    let mut arduino_serial = serialport::new(port_name, 115200).open()
        .expect("[ERR] Failed to open port");
    // if serial data isn't constantly streaming, we time out.
    arduino_serial.set_timeout(Duration::new(1000000000,0))
        .expect("failed to set new timeout");

    loop {
        let mut buf = [0;mem::size_of::<CanFrame>()];
        arduino_serial.read(&mut buf)
            .expect("fail to get id");

        let frame = populate_canframe(&buf);

        match sensor_listeners.get(&frame.id) {
            Some(func) => func(frame.data),
            None => println!("No Entry for this ID")
        };
    }
}
