use serialport;
use std::env;
use std::time::Duration;
use std::collections::HashMap;
use std::mem;
//use std::thread;

#[path = "../lib/canframe.rs"] mod canframe;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut port_name = String::new();
    if args.len() > 1 {
        port_name = args[1].clone();
    }

    let mut known_sensors: HashMap<u32, u64> = HashMap::new();

    let ports = serialport::available_ports().unwrap();
    if port_name == "" {
        for i in ports.iter() {
            println!("{:?}",i);
            port_name = i.port_name.clone();
        }
    }
    println!("[INFO] reading from {}", port_name);
    let mut arduino_serial = serialport::new(port_name, 115200).open()
        .expect("[ERRO] Failed to open port");
    // if serial data isn't constantly streaming, we time out.
    arduino_serial.set_timeout(Duration::new(1000000000,0))
        .expect("[ERRO] failed to set new timeout");

    loop {
        let mut buf = [0;mem::size_of::<canframe::CanFrame>()];
        arduino_serial.read(&mut buf)
            .expect("[ERRO] Failed to get id");
        let frame = canframe::populate_canframe(&buf);
        let data = known_sensors.entry(frame.id).or_insert(u64::from_be_bytes(frame.data));
        println!("{:#010x} : {:#018x}", frame.id, data);
    }

}
