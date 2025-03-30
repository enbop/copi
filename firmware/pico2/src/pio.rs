use embassy_rp::{
    peripherals::{PIO0, PIO1, PIO2},
    pio::{LoadedProgram, Pio},
};

use crate::Irqs;

pub struct PioControl<'d> {
    pub pio0: Pio<'d, PIO0>,
    pub pio1: Pio<'d, PIO1>,
    pub pio2: Pio<'d, PIO2>,

    pub program0: Option<LoadedProgram<'d, PIO0>>,
    pub program1: Option<LoadedProgram<'d, PIO1>>,
    pub program2: Option<LoadedProgram<'d, PIO2>>,
}

impl PioControl<'_> {
    pub unsafe fn init() -> Self {
        let rp = unsafe { embassy_rp::Peripherals::steal() };

        Self {
            pio0: Pio::new(rp.PIO0, Irqs),
            pio1: Pio::new(rp.PIO1, Irqs),
            pio2: Pio::new(rp.PIO2, Irqs),
            program0: None,
            program1: None,
            program2: None,
        }
    }
}

#[macro_export]
macro_rules! pio_run_with_program {
    ($pio_control:expr, $num:expr, $run:expr) => {
        match $num {
            0 => $run(&mut $pio_control.pio0, &mut $pio_control.program0),
            1 => $run(&mut $pio_control.pio1, &mut $pio_control.program1),
            2 => $run(&mut $pio_control.pio2, &mut $pio_control.program2),
            _ => panic!("Invalid PIO number"),
        };
    };
}

#[macro_export]
macro_rules! pio_run {
    ($pio_control:expr, $num:expr, $run:expr) => {
        match $num {
            0 => $run(&mut $pio_control.pio0),
            1 => $run(&mut $pio_control.pio1),
            2 => $run(&mut $pio_control.pio2),
            _ => panic!("Invalid PIO number"),
        };
    };
    ($pio_control:ident, $num:expr, $run:expr, $($args:expr),*) => {
        match $num {
            0 => $run(&mut $pio_control.pio0, $($args),*),
            1 => $run(&mut $pio_control.pio1, $($args),*),
            2 => $run(&mut $pio_control.pio2, $($args),*),
            _ => panic!("Invalid PIO number"),
        };
    };
}

#[macro_export]
macro_rules! sm_invoke {
    ($pio:expr, $num:expr, $method:ident) => {
        match $num {
            0 => $pio.sm0.$method(),
            1 => $pio.sm1.$method(),
            2 => $pio.sm2.$method(),
            _ => panic!("Invalid PIO number"),
        }
    };
    ($pio:expr, $num:expr, $method:ident, $($args:expr),*) => {
        match $num {
            0 => $pio.sm0.$method($($args),*),
            1 => $pio.sm1.$method($($args),*),
            2 => $pio.sm2.$method($($args),*),
            _ => panic!("Invalid PIO number"),
        }
    };
}

#[macro_export]
macro_rules! sm_run {
    ($pio:expr, $num:expr, $run:expr) => {
        match $num {
            0 => $run(&mut $pio.sm0),
            1 => $run(&mut $pio.sm1),
            2 => $run(&mut $pio.sm2),
            _ => panic!("Invalid number"),
        }
    };
}

#[macro_export]
macro_rules! pio_sm_invoke {
    ($pio_control:expr, $pio_num:expr, $sm_num: expr, $method:ident) => {
        match $pio_num {
            0 => sm_invoke!($pio_control.pio0, $sm_num, $method),
            1 => sm_invoke!($pio_control.pio1, $sm_num, $method),
            2 => sm_invoke!($pio_control.pio2, $sm_num, $method),
            _ => panic!("Invalid PIO number"),
        }
    };
    ($pio_control:expr, $pio_num:expr, $sm_num: expr, $method:ident, $($args:expr),*) => {
        match $pio_num {
            0 => sm_invoke!($pio_control.pio0, $sm_num, $method, $($args),*),
            1 => sm_invoke!($pio_control.pio1, $sm_num, $method, $($args),*),
            2 => sm_invoke!($pio_control.pio2, $sm_num, $method, $($args),*),
            _ => panic!("Invalid PIO number"),
        }
    }

}

#[macro_export]
macro_rules! pio_sm_run {
    ($pio_control:expr, $pio_num:expr, $sm_num: expr, $run:expr) => {
        match $pio_num {
            0 => sm_run!($pio_control.pio0, $sm_num, $run),
            1 => sm_run!($pio_control.pio1, $sm_num, $run),
            2 => sm_run!($pio_control.pio2, $sm_num, $run),
            _ => panic!("Invalid number"),
        };
    };
}
