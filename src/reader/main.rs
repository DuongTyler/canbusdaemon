use serialport;
use std::env;
use std::time::Duration;
use std::collections::HashMap;
use std::mem;
//use std::thread;

use gtk;
use glib;
use gtk::prelude::*;

#[path = "../lib/canframe.rs"] mod canframe;

fn build_ui (app: &gtk::Application) {
    let builder: gtk::Builder = gtk::Builder::from_file("window.glade");
    let window: gtk::Window = builder.object("main_window").expect("unable to get main window from glade");
    let entities: gtk::ListBox = gtk::ListBox::new();
    
    window.set_application(Some(app));
    window.show_all();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut port_name = String::new();
    if args.len() > 1 {
        port_name = args[1].clone();
    }

    let mut known_sensors = HashMap::new();

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

    let app = gtk::Application::builder()
        .application_id("com.canbusreader.CanbusReader")
        .build();

    app.connect_activate(|app| {
        build_ui(&app);
    });

    glib::idle_add(move || {
        //if there's nothing to read, continue or we'll be blocking the main thread
        if arduino_serial.bytes_to_read().unwrap() <= 0 {
            return Continue(true);
        }
        let mut buf = [0;mem::size_of::<canframe::CanFrame>()];
        arduino_serial.read(&mut buf)
            .expect("[ERRO] Failed to get id");
        let frame = canframe::populate_canframe(&buf);
        known_sensors.insert(frame.id, 64);
        println!("{}", frame.id);
        return Continue(true);
    });

    //thread::spawn(move || {
    //    loop {
    //        let mut buf = [0;mem::size_of::<canframe::CanFrame>()];
    //        arduino_serial.read(&mut buf)
    //            .expect("[ERRO] Failed to get id");
    //        let frame = canframe::populate_canframe(&buf);
    //        known_sensors.insert(frame.id, 64);
    //    }
    //});

    app.run();
}
