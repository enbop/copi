use defmt::{panic, unwrap};
use embassy_executor::Spawner;
use embassy_rp::{peripherals::USB, usb::Driver};
use embassy_usb::{
    UsbDevice,
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

pub fn init(
    spawner: Spawner,
    p: embassy_rp::Peripherals,
) -> CdcAcmClass<'static, Driver<'static, USB>> {
    // Create the driver, from the HAL.
    let driver = Driver::new(p.USB, crate::Irqs);

    // Create embassy-usb Config
    let config = {
        let mut config = embassy_usb::Config::new(0x9527, 0xacdc);
        config.manufacturer = Some("Enbop");
        config.product = Some("Copi");
        config.serial_number = Some("88489527");
        config.max_power = 100;
        config.max_packet_size_0 = 64;
        config
    };

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut builder = {
        static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

        let builder = embassy_usb::Builder::new(
            driver,
            config,
            CONFIG_DESCRIPTOR.init([0; 256]),
            BOS_DESCRIPTOR.init([0; 256]),
            &mut [], // no msos descriptors
            CONTROL_BUF.init([0; 64]),
        );
        builder
    };

    // Create classes on the builder.
    let class = {
        static STATE: StaticCell<State> = StaticCell::new();
        let state = STATE.init(State::new());
        CdcAcmClass::new(&mut builder, state, 64)
    };

    // Build the builder.
    let usb = builder.build();

    // Run the USB device.
    unwrap!(spawner.spawn(usb_task(usb)));

    class
}

type MyUsbDriver = Driver<'static, USB>;
type MyUsbDevice = UsbDevice<'static, MyUsbDriver>;

#[embassy_executor::task]
async fn usb_task(mut usb: MyUsbDevice) -> ! {
    usb.run().await
}

pub struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}
