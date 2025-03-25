use embassy_rp::{
    Peripheral,
    gpio::{AnyPin, Input, Output, Pin as _},
};

pub struct Pin {
    pin: AnyPin,
    state: PinState,
    resource_index: usize,
}

impl Pin {
    pub fn new(pin: AnyPin) -> Self {
        Self {
            pin,
            state: PinState::None,
            resource_index: 0,
        }
    }
}

#[derive(PartialEq)]
pub enum PinState {
    None,
    GpioInput,
    GpioOutput,
    PwmOut,
    PwmIn,
}

pub struct Slot<T, const N: usize> {
    array: [Option<T>; N],
    size: u8,
}

impl<T, const N: usize> Slot<T, N> {
    pub fn new() -> Self {
        Self {
            array: [const { None }; N],
            size: 0,
        }
    }

    pub fn len(&self) -> u8 {
        self.size
    }

    // returns index of the pins
    pub fn add(&mut self, pin: T) -> Option<usize> {
        if self.size >= 16 {
            return None;
        }
        for i in 0..self.array.len() {
            if self.array[i].is_none() {
                self.array[i] = Some(pin);
                assert_eq!(self.size as usize, i);
                self.size += 1;
                return Some(i);
            }
        }
        unreachable!();
    }

    pub fn remove(&mut self, index: usize) -> bool {
        if index >= self.array.len() {
            return false;
        }
        let old = self.array[index].take();
        if old.is_none() {
            return false;
        }
        assert!(self.size > 0);
        self.size -= 1;
        true
    }

    // pub fn contains_pin(&self, pin_num: u8) -> bool {
    //     for pin in self.pins.as_ref() {
    //         let Some(pin) = pin else {
    //             break;
    //         };
    //         if pin.pin_num() == pin_num {
    //             return true;
    //         }
    //     }
    //     false
    // }

    // pub fn get_pin(&self, index: u8) -> Option<u8> {
    //     if index >= self.size {
    //         return None;
    //     }
    //     Some(self.pins[index as usize])
    // }

    // pub fn get_all(&self) -> &[u8] {
    //     &self.pins[..self.size as usize]
    // }
}

pub struct PeripheralController<'d> {
    embassy_rp: embassy_rp::Peripherals,
    pins: [Pin; 30],

    gpio_output: Slot<Output<'d>, 30>,
}

impl PeripheralController<'_> {
    pub fn new() -> Self {
        let this = unsafe {
            let embassy_rp = embassy_rp::Peripherals::steal();
            Self {
                // TODO use macros to generate this
                pins: [
                    Pin::new(embassy_rp.PIN_0.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_1.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_2.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_3.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_4.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_5.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_6.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_7.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_8.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_9.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_10.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_11.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_12.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_13.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_14.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_15.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_16.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_17.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_18.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_19.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_20.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_21.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_22.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_23.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_24.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_25.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_26.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_27.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_28.clone_unchecked().degrade()),
                    Pin::new(embassy_rp.PIN_29.clone_unchecked().degrade()),
                ],
                embassy_rp,
                gpio_output: Slot::new(),
            }
        };
        this
    }

    pub fn gpio_output_init(&mut self, pin_num: usize, value: bool) -> bool {
        let pin = &mut self.pins[pin_num];
        if pin.state != PinState::None {
            return false;
        }

        unsafe {
            let pin_cl = pin.pin.clone_unchecked();
            let output = Output::new(
                pin_cl,
                if value {
                    embassy_rp::gpio::Level::High
                } else {
                    embassy_rp::gpio::Level::Low
                },
            );
            let Some(index) = self.gpio_output.add(output) else {
                // TODO: handle error
                return false;
            };
            pin.resource_index = index;
            pin.state = PinState::GpioOutput;
            true
        }
    }

    // TODO return a Result instead of bool
    pub fn gpio_output_set(&mut self, pin_num: usize, value: bool) -> bool {
        let pin = &mut self.pins[pin_num];
        if pin.state != PinState::GpioOutput {
            return false;
        }
        let output = &mut self.gpio_output.array[pin.resource_index];
        assert!(output.is_some());
        if value {
            output.as_mut().unwrap().set_high();
        } else {
            output.as_mut().unwrap().set_low();
        }
        true
    }
}
