use std::io::Write;

use copi_protocol::Command;
use tokio_serial::SerialStream;

pub mod gpio;
pub mod pio;
pub mod pwm;

const MAX_USB_PACKET_SIZE: usize = 64;

pub fn send_command(port: &mut SerialStream, cmd: Command) {
    let mut buf = [0u8; MAX_USB_PACKET_SIZE];
    let len = minicbor::len(&cmd);
    minicbor::encode(&cmd, buf.as_mut()).unwrap();

    // TODO should be async?
    match port.write_all(&buf[..len]) {
        Ok(_) => {
            log::info!("Sent command: {:?}", cmd);
        }
        Err(e) => {
            log::error!("Failed to send command: {:?}", e);
        }
    }
}
