use serialport;
use std::env;
use std::time::Duration;
use std::collections::HashMap;
use std::mem;

#[path = "../lib/canframe.rs"] mod canframe;

struct SensorNumMap {
    _data: u64,
    index: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut port_name = String::new();
    if args.len() > 1 {
        port_name = args[1].clone();
    }

    let mut known_sensors: HashMap<u32, SensorNumMap> = HashMap::new();

    println!("[INFO] reading from {}", port_name);
    let mut arduino_serial = serialport::new(port_name, 115200).open()
        .expect("[ERRO] Failed to open port");
    // if serial data isn't constantly streaming, we time out.
    arduino_serial.set_timeout(Duration::new(1000000000,0))
        .expect("[ERRO] failed to set new timeout");
    print!("\x1B[2J");
    loop {
        let mut buf = [0;mem::size_of::<canframe::CanFrame>()];
        arduino_serial.read(&mut buf)
            .expect("[ERRO] Failed to get id");
        let frame = canframe::populate_canframe(&buf);

        let len: usize = known_sensors.len();
        let mut _index: usize = 1;
        match known_sensors.get(&frame.id) {
            Some(x) => _index = x.index, //already has an index
            None => {
                _index = len;    //place new entry at end of list
                known_sensors
                    .insert(frame.id, 
                        SensorNumMap{
                            _data: u64::from_be_bytes(frame.data),
                            index: len,
                            });
            },
        }

        
        print!("\x1B[{};1H", _index+1);  //move cursor to row index+1 col 1
        println!("{:#010x} : {:#018x} ({:3})", frame.id, u64::from_be_bytes(frame.data), _index);    //update entry in position
    }
}
