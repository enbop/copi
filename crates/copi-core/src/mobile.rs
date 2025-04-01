use copi_protocol::Command;
use nusb::transfer::Direction;
use nusb::{DeviceInfo, InterfaceInfo};
use tokio::sync::mpsc::UnboundedReceiver;

use crate::MAX_USB_PACKET_SIZE;

const USB_INTR_CLASS_COMM: u8 = 0x02;
const USB_INTR_SUBCLASS_ACM: u8 = 0x02;
const USB_INTR_CLASS_CDC_DATA: u8 = 0x0A;

#[allow(unreachable_code)]
#[allow(unused_variables)]
// https://github.com/wuwbobo2021/android-usbser-rs
pub async fn start_usb_cdc_service(fd: i32, mut cmd_rx: UnboundedReceiver<Command>) {
    let di = nusb::list_devices()
        .unwrap()
        .find(|d| d.vendor_id() == 49374 && d.product_id() == 51966)
        .expect("device should be connected");

    log::info!("Device info: {:?}", di);

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

    let (intr_comm, intr_data) = find_interfaces(&di).unwrap();
    let intr_comm = device
        .detach_and_claim_interface(intr_comm.interface_number())
        .unwrap();
    let intr_data = device
        .detach_and_claim_interface(intr_data.interface_number())
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

/// (android_usbser)
/// Returns (intr_comm, intr_data) if it is a CDC-ACM device.
fn find_interfaces(dev_info: &DeviceInfo) -> Option<(InterfaceInfo, InterfaceInfo)> {
    let (comm, data) = (
        dev_info.interfaces().find(|intr| {
            intr.class() == USB_INTR_CLASS_COMM && intr.subclass() == USB_INTR_SUBCLASS_ACM
        }),
        dev_info
            .interfaces()
            .find(|intr| intr.class() == USB_INTR_CLASS_CDC_DATA),
    );
    if let (Some(comm), Some(data)) = (comm, data) {
        Some((comm.clone(), data.clone()))
    } else {
        None
    }
}
