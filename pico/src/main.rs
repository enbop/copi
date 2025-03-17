#![no_std]
#![no_main]

use defmt::{info, panic, unwrap};
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::peripherals::{PIN_4, PWM_SLICE2};
use embassy_rp::pwm::{Config, Pwm, SetDutyCycle};
use embassy_rp::usb::{Driver, Instance, InterruptHandler};
use embassy_time::Timer;
use embassy_usb::UsbDevice;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;
use copi_command;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // If we aim for a specific frequency, here is how we can calculate the top value.
    // The top value sets the period of the PWM cycle, so a counter goes from 0 to top and then wraps around to 0.
    // Every such wraparound is one PWM cycle. So here is how we get 25KHz:
    let desired_freq_hz = 25_000;
    let clock_freq_hz = embassy_rp::clocks::clk_sys_freq();
    let divider = 16u8;
    let period = (clock_freq_hz / (desired_freq_hz * divider as u32)) as u16 - 1;

    let mut c = Config::default();
    c.top = period;
    c.divider = divider.into();
    let mut pwm: Pwm<'_> = Pwm::new_output_a(p.PWM_SLICE2, p.PIN_4, c.clone());

    pwm.set_duty_cycle_percent(66).unwrap();

    // Create the driver, from the HAL.
    let driver = Driver::new(p.USB, Irqs);

    // Create embassy-usb Config
    let config = {
        let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
        config.manufacturer = Some("Embassy");
        config.product = Some("USB-serial example");
        config.serial_number = Some("12345678");
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
    let mut class = {
        static STATE: StaticCell<State> = StaticCell::new();
        let state = STATE.init(State::new());
        CdcAcmClass::new(&mut builder, state, 64)
    };

    // Build the builder.
    let usb = builder.build();

    // Run the USB device.
    unwrap!(spawner.spawn(usb_task(usb)));

    // Do stuff with the class!
    loop {
        class.wait_connection().await;
        info!("Connected");
        let _ = handle_class(&mut class, &mut pwm).await;
        info!("Disconnected");
    }
}

type MyUsbDriver = Driver<'static, USB>;
type MyUsbDevice = UsbDevice<'static, MyUsbDriver>;

#[embassy_executor::task]
async fn usb_task(mut usb: MyUsbDevice) -> ! {
    usb.run().await
}

struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}

async fn handle_class<'d, T: Instance + 'd>(
    class: &mut CdcAcmClass<'d, Driver<'d, T>>,
    pwm: &mut Pwm<'_>,
) -> Result<(), Disconnected> {
    let mut buf = [0; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];
        info!("data: {} - {:x}", n, data);

        if let Ok(command) = minicbor::decode::<copi_command::Command>(data) {
            match command {
                copi_command::Command::SetPWM {
                    name,
                    period,
                    duty_cycle,
                    percent,
                } => {
                    info!("SetPWM: {} {} {} {}", name, period, duty_cycle, percent);
                    pwm.set_duty_cycle_percent(percent).unwrap();
                    // let pwm = Pwm::new(PWM_SLICE2, Config::default());
                    // pwm.set_period(period);
                    // pwm.set_duty_cycle(SetDutyCycle::new(name, duty_cycle));
                }
                copi_command::Command::SetGPIO { pin, state } => {
                    info!("SetGPIO: {} {}", pin, state);
                    // let pin = PIN_4;
                    // pin.set_high();
                }
            }
        };

        // echo
        // class.write_packet(data).await?;
        // info!("echoed");
    }
}
