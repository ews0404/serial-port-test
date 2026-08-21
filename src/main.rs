use std::time::{Duration, Instant};

use serialport;

fn main() {
    println!("\n\nStarting serial-port-test...");

    // check what serial ports are available
    match serialport::available_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                println!("no serial ports detected, exiting");
                return;
            } else {
                println!("available ports:");
                for p in ports.iter() {
                    println!("   {}", p.port_name);
                }
            }
        }
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    // open first available port
    let port_info = &serialport::available_ports().unwrap()[0];
    print!("opening {}...", port_info.port_name);
    let mut port = serialport::new(&port_info.port_name, 115200)
        .data_bits(serialport::DataBits::Eight)
        .stop_bits(serialport::StopBits::One)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("serial open error");
    println!("done!");

    // write values out of the port and read them back in
    let mut tx_buf: [u8; 1] = [0u8; 1];
    let mut rx_buf: [u8; 1] = [0u8; 1];
    let mut good = 0;
    let mut bad = 0;
    let wait_millis = Duration::from_millis(1);
    let mut time = Instant::now();
    let mut keep_going = true;
    
    while keep_going{

        // send a set of 255 chars
        for i in 0..255 {
            // wait for next tick
            while Instant::now() - time < wait_millis {}
            time = Instant::now();

            // send a value
            tx_buf[0] = i;
            port.write(&tx_buf).unwrap();

            // // wait for data to show up
            // while port.bytes_to_read().unwrap() == 0 {}

            // read value back
            port.read(&mut rx_buf).unwrap();

            // did we get back what we sent out?
            if tx_buf[0] == rx_buf[0] {
                good += 1;
            } else {
                bad += 1;
            }
        }

        // summarize results
        println!("errors: {}/{}", bad, good);
    }
}
