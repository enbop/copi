#![no_std]
#![no_main]
#![feature(generic_arg_infer)] // https://blog.rust-lang.org/inside-rust/2025/03/05/inferred-const-generic-arguments.html

mod peripherals;
mod pio;
mod usb;

use ::pio::{ArrayVec, PioVersion, Program, SideSet, Wrap};
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
    use copi_protocol::Command::*;
    let mut buf = [0; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];
        info!("data: {} - {:x}", n, data);

        if let Ok(command) = minicbor::decode::<copi_protocol::Command>(data) {
            match command {
                GpioOutputInit { rid, pin, value } => {
                    info!("GpioOutputInit: {} {}", pin, value);
                    pc.gpio_output_init(pin as _, value);
                }
                GpioOutputSet { rid, pin, state } => {
                    info!("GpioOutputSet: {} {}", pin, state);
                    pc.gpio_output_set(pin as _, state);
                }
                PwmInit {
                    rid,
                    slice,
                    a,
                    b,
                    divider,
                    compare_a,
                    compare_b,
                    top,
                } => {
                    info!(
                        "PwmInit: {} {} {} {} {} {} {}",
                        slice,
                        a.unwrap_or(0),
                        b.unwrap_or(0),
                        divider,
                        compare_a,
                        compare_b,
                        top
                    );
                    pc.pwm_init(slice, a, b, divider, compare_a, compare_b, top);
                }
                PwmSetDutyCyclePercent { rid, pin, percent } => {
                    info!("PwmSetDutyCyclePercent: {} {} {}", rid, pin, percent);
                    pc.pwm_set_duty_cycle_percent(pin, percent);
                }
                PioLoadProgram {
                    rid,
                    pio_num,
                    program,
                    program_len,
                    origin,
                    wrap_source,
                    wrap_target,
                    side_set_opt,
                    side_set_bits,
                    side_set_pindirs,
                    pio_version_v0,
                } => {
                    let mut program_u16 = [0u16; 16];
                    for i in 0..16 {
                        program_u16[i] = u16::from_le_bytes([program[i * 2], program[i * 2 + 1]]);
                    }
                    let program_u16_len = program_len / 2;

                    let mut code = ArrayVec::<u16, 16>::new();
                    for i in &program_u16[..program_u16_len as _] {
                        code.push(*i);
                    }
                    pc.pio_load_program(
                        pio_num as _,
                        Program {
                            code,
                            origin,
                            wrap: Wrap {
                                source: wrap_source,
                                target: wrap_target,
                            },
                            side_set: SideSet::new(
                                side_set_opt,
                                side_set_bits - 1,
                                side_set_pindirs,
                            ),
                            version: if pio_version_v0 {
                                PioVersion::V0
                            } else {
                                PioVersion::V1
                            },
                        },
                    );
                }
                PioSmInit {
                    rid,
                    pio_num,
                    sm_num,
                    pin_num,
                } => {
                    info!("PioSmInit: {} {} {}", rid, pio_num, sm_num);
                    pc.pio_sm_init(pio_num as _, sm_num as _, pin_num as _);
                }
                PioSmSetEnable {
                    rid,
                    pio_num,
                    sm_num,
                    enable,
                } => {
                    info!("PioSmSetEnable: {} {} {}", rid, pio_num, enable);
                    pc.pio_sm_set_enable(pio_num as _, sm_num as _, enable);
                }
                PioSmPush {
                    rid,
                    pio_num,
                    sm_num,
                    instr,
                } => {
                    info!("PioPush: {} {} {}", rid, pio_num, data);
                    pc.pio_sm_push(pio_num as _, sm_num as _, instr);
                }
                PioSmExecInstr {
                    rid,
                    pio_num,
                    sm_num,
                    exec_instr,
                } => {
                    info!(
                        "PioExecInstr: {} {} {} {}",
                        rid, pio_num, sm_num, exec_instr
                    );
                    unsafe {
                        pc.pio_sm_exec_instr_unchecked(pio_num as _, sm_num as _, exec_instr);
                    }
                }
                _ => {
                    info!("Unknown command");
                }
            }
        };

        // echo
        // class.write_packet(data).await?;
        // info!("echoed");
    }
}
