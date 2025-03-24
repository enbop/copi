use embassy_rp::gpio::{AnyPin, Output, Pin};

pub struct OutputPin {
    pin: AnyPin,
    output: Output<'static>,
}

trait PinNum {
    fn pin_num(&self) -> u8;
}

impl PinNum for OutputPin {
    fn pin_num(&self) -> u8 {
        self.pin.pin()
    }
}

pub struct PinArray<T: PinNum> {
    pins: [Option<T>; 16],
    size: u8,
}

impl<T: PinNum> PinArray<T> {
    pub fn new() -> Self {
        Self {
            pins: [const { None }; 16],
            size: 0,
        }
    }

    pub fn len(&self) -> u8 {
        self.size
    }

    pub fn add_pin(&mut self, pin: T) -> bool {
        if self.size >= 16 {
            return false;
        }
        for i in 0..self.pins.len() {
            if self.pins[i].is_none() {
                self.pins[i] = Some(pin);
                assert_eq!(self.size as usize, i);
                self.size += 1;
                return true;
            }
        }
        unreachable!();
    }

    pub fn remove_pin(&mut self, pin_num: u8) -> bool {
        for i in 0..self.pins.len() {
            let Some(pin) = self.pins[i].as_ref() else {
                return false;
            };

            if pin.pin_num() == pin_num {
                self.pins[i] = None;
                self.reshape();
                return true;
            }
        }
        false
    }

    pub fn contains_pin(&self, pin_num: u8) -> bool {
        for pin in self.pins.as_ref() {
            let Some(pin) = pin else {
                break;
            };
            if pin.pin_num() == pin_num {
                return true;
            }
        }
        false
    }

    fn reshape(&mut self) {
        let mut new_pins = [const { None }; 16];
        let mut new_size = 0;

        for i in 0..self.pins.len() {
            if self.pins[i].is_some() {
                new_pins[new_size] = core::mem::take(&mut self.pins[i]);
                new_size += 1;
            }
        }
        self.pins = new_pins;
        self.size = new_size as u8;
    }

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

pub struct GpioOutputControl {
    pins: PinArray<OutputPin>,
}

impl GpioOutputControl {
    pub fn new() -> Self {
        Self {
            pins: PinArray::new(),
        }
    }

    pub fn init_pin(&mut self, pin_num: u8, value: bool) -> bool {
        // Initialize the pin as output, save the pin number

        todo!()
    }
}
