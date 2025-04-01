uniffi::include_scaffolding!("export");

use log::LevelFilter;
use log::info;
use nusb::transfer::Direction;
use nusb::{DeviceInfo, InterfaceInfo};
use std::thread::sleep;
use std::time::Duration;

enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Into<LevelFilter> for LogLevel {
    fn into(self) -> LevelFilter {
        match self {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[allow(unused_variables)]
fn init_logger(level: LogLevel) {
    #[cfg(target_os = "android")]
    android_logger::init_once(android_logger::Config::default().with_max_level(level.into()));
}

// https://github.com/wuwbobo2021/android-usbser-rs
#[allow(unreachable_code)]
#[allow(unused_variables)]
fn init_usb_fd(fd: i32) {
    info!("Try to connect {}", fd);

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

    loop {
        writer.submit(vec![0x01, 0x02, 0x03, 0x04]);
        sleep(Duration::from_millis(2000));
    }
}

const USB_INTR_CLASS_COMM: u8 = 0x02;
const USB_INTR_SUBCLASS_ACM: u8 = 0x02;
const USB_INTR_CLASS_CDC_DATA: u8 = 0x0A;

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
