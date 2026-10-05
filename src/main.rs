use core::time;
use std::{thread, time::{Duration}};

use serialport;

fn main() {
    println!("\n\nStarting serial-port-test...");

    let dt=time::Duration::from_millis(100);

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
    println!("done!\n");
    

    // write values out of the port and read them back in
    let mut tx_buf: [u8; 1] = [0u8; 1];
    let mut rx_buf: [u8; 1] = [0u8; 1];

    loop {

        // send a set of 255 chars
        for i in 0..255 {

            // send a value
            tx_buf[0] = i;
            match port.write(&tx_buf){
                Ok(_)=>{}
                Err(err)=>{println!("txErr: {:?}", err);}
            }
            thread::sleep(dt);

            // wait for data to show up
            let mut ix=0;
            while ix==0 {
                match port.bytes_to_read(){
                    Ok(x)=>{ if x!=0 {
                        ix=x; 
                        println!("bytes:{0}", x); 
                    }}
                    Err(_err)=>{}
                }
            }
            while port.bytes_to_read().unwrap() == 0 {}

            // read value back
            match port.read(&mut rx_buf){
                Ok(_)=>{println!("{0}:{1}:{2}", i, &tx_buf[0],  &rx_buf[0]);}
                Err(err)=>{println!("rxErr: {:?}", err);}
            }
        }
    }
}
