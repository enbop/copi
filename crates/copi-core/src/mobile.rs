use crate::MAX_USB_PACKET_SIZE;
use crate::generated::*;
use nusb::transfer::{Direction, RequestBuffer};
use prost::Message as _;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[allow(unreachable_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
// https://github.com/wuwbobo2021/android-usbser-rs
pub async fn start_usb_cdc_service(
    fd: i32,
    interface_comm: i32,
    interface_data: i32,
    mut request_rx: UnboundedReceiver<CopiRequest>,
    response_tx: UnboundedSender<CopiResponse>,
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
    let mut reader = intr_data.bulk_in_queue(addr_r.unwrap());
    let mut writer = intr_data.bulk_out_queue(addr_w.unwrap());

    // init reader
    let n_transfers = 8;
    let transfer_size = 256;
    while reader.pending() < n_transfers {
        reader.submit(RequestBuffer::new(transfer_size));
    }

    let mut request_buf = [0u8; MAX_USB_PACKET_SIZE];
    let mut response_buf = [0u8; MAX_USB_PACKET_SIZE];

    log::info!("USB CDC service started");
    loop {
        tokio::select! {
            req = request_rx.recv() => {
                log::info!("Received request: {:?}", req);
                // TODO use buf
                // TODO check size
                if let Some(cmd) = req {
                    writer.submit(cmd.encode_to_vec().to_vec());
                } else {
                    log::warn!("Command receiver closed");
                    break;
                }
            }
            res = reader.next_complete() => {
                log::info!("Received response: {:?}", res);
                    if res.status.is_err() {
                        log::error!("Failed to read response: {:?}", res.status);
                        break;
                }

                let response = CopiResponse::decode(&response_buf[..]);
                match response {
                    Ok(resp) => {
                        log::info!("Received response: {:?}", resp);
                        if response_tx.send(resp).is_err() {
                            log::warn!("Failed to send response to receiver");
                            break;
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to decode response: {:?}", e);
                    }
                }
                reader.submit(RequestBuffer::reuse(res.data, transfer_size))
            }
        }
    }

    log::info!("USB CDC service stopped");
}
