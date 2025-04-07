use copi_protocol::Command;
use nusb::transfer::Direction;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::MAX_USB_PACKET_SIZE;

#[allow(unreachable_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
// https://github.com/wuwbobo2021/android-usbser-rs
pub async fn start_usb_cdc_service(
    fd: i32,
    interface_comm: i32,
    interface_data: i32,
    mut cmd_rx: UnboundedReceiver<Command>,
) {
    // (android_usbser)
    // Safety: `close()` is not called automatically when the JNI `AutoLocal` of `conn`
    // and the corresponding Java object is destroyed. (check `UsbDeviceConnection` source)
    #[cfg(target_os = "android")]
    let device: nusb::Device = unsafe {
        use std::os::fd::*;
        let owned_fd = unsafe { OwnedFd::from_raw_fd(fd as RawFd) };
        nusb::Device::from_fd(owned_fd).unwrap()
    };
    #[cfg(not(target_os = "android"))]
    let device: nusb::Device = unreachable!();

    let intr_comm = device
        .detach_and_claim_interface(interface_comm as _)
        .unwrap();
    let intr_data = device
        .detach_and_claim_interface(interface_data as _)
        .unwrap();

    // Note: It doesn't select a setting with the highest bandwidth.
    let (mut addr_r, mut addr_w) = (None, None);
    for alt in intr_data.descriptors() {
        let endps: Vec<_> = alt.endpoints().collect();
        let endp_r = endps.iter().find(|endp| endp.direction() == Direction::In);
        let endp_w = endps.iter().find(|endp| endp.direction() == Direction::Out);
        if endp_r.is_some() && endp_w.is_some() {
            addr_r = Some(endp_r.unwrap().address());
            addr_w = Some(endp_w.unwrap().address());
            break;
        }
    }
    let reader = intr_data.bulk_in_queue(addr_r.unwrap());
    let mut writer = intr_data.bulk_out_queue(addr_w.unwrap());

    let mut buf = [0u8; MAX_USB_PACKET_SIZE];
    loop {
        match cmd_rx.recv().await {
            Some(cmd) => {
                let len = minicbor::len(&cmd);
                minicbor::encode(&cmd, buf.as_mut()).unwrap();
                writer.submit(buf[..len].to_vec())
            }
            None => {
                log::warn!("Command receiver closed");
                break;
            }
        }
    }
}
