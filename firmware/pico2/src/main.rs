#![no_std]
#![no_main]
#![feature(generic_arg_infer)] // https://blog.rust-lang.org/inside-rust/2025/03/05/inferred-const-generic-arguments.html

mod command;
mod peripherals;
mod pio;
mod usb;
mod utils;

use command::handle_command;
use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    peripherals::{PIO0, PIO1, PIO2, USB},
    usb::{Driver, Instance},
};
use embassy_usb::class::cdc_acm::CdcAcmClass;
use peripherals::PeripheralController;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<PIO0>;
    PIO1_IRQ_0 => embassy_rp::pio::InterruptHandler<PIO1>;
    PIO2_IRQ_0 => embassy_rp::pio::InterruptHandler<PIO2>;
});

// Program metadata for `picotool info`.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"copi-firmware"),
    embassy_rp::binary_info::rp_program_description!(c""),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p: embassy_rp::Peripherals = embassy_rp::init(Default::default());

    let mut pc = PeripheralController::new();

    let mut class = usb::init(spawner, p);

    loop {
        class.wait_connection().await;
        info!("USB Connected");
        let _ = handle_class(&mut class, &mut pc).await;
        info!("USB Disconnected");
    }
}

async fn handle_class<'d, T: Instance + 'd>(
    class: &mut CdcAcmClass<'d, Driver<'d, T>>,
    pc: &mut PeripheralController<'d>,
) -> Result<(), usb::Disconnected> {
    let mut buf = [0; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];
        info!("data: {} - {:x}", n, data);

        if let Ok(command) = minicbor::decode::<copi_protocol::Command>(data) {
            handle_command(pc, command);
        };
    }
}
