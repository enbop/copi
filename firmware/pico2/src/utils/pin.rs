use embassy_rp::{
    Peripheral as _,
    gpio::{AnyPin, Pin as _},
};

pub unsafe fn get_anypin_unchecked(rp: &embassy_rp::Peripherals, pin_num: u8) -> AnyPin {
    unsafe {
        match pin_num {
            0 => rp.PIN_0.clone_unchecked().degrade(),
            1 => rp.PIN_1.clone_unchecked().degrade(),
            2 => rp.PIN_2.clone_unchecked().degrade(),
            3 => rp.PIN_3.clone_unchecked().degrade(),
            4 => rp.PIN_4.clone_unchecked().degrade(),
            5 => rp.PIN_5.clone_unchecked().degrade(),
            6 => rp.PIN_6.clone_unchecked().degrade(),
            7 => rp.PIN_7.clone_unchecked().degrade(),
            8 => rp.PIN_8.clone_unchecked().degrade(),
            9 => rp.PIN_9.clone_unchecked().degrade(),
            10 => rp.PIN_10.clone_unchecked().degrade(),
            11 => rp.PIN_11.clone_unchecked().degrade(),
            12 => rp.PIN_12.clone_unchecked().degrade(),
            13 => rp.PIN_13.clone_unchecked().degrade(),
            14 => rp.PIN_14.clone_unchecked().degrade(),
            15 => rp.PIN_15.clone_unchecked().degrade(),
            16 => rp.PIN_16.clone_unchecked().degrade(),
            17 => rp.PIN_17.clone_unchecked().degrade(),
            18 => rp.PIN_18.clone_unchecked().degrade(),
            19 => rp.PIN_19.clone_unchecked().degrade(),
            20 => rp.PIN_20.clone_unchecked().degrade(),
            21 => rp.PIN_21.clone_unchecked().degrade(),
            22 => rp.PIN_22.clone_unchecked().degrade(),
            23 => rp.PIN_23.clone_unchecked().degrade(),
            24 => rp.PIN_24.clone_unchecked().degrade(),
            25 => rp.PIN_25.clone_unchecked().degrade(),
            26 => rp.PIN_26.clone_unchecked().degrade(),
            27 => rp.PIN_27.clone_unchecked().degrade(),
            28 => rp.PIN_28.clone_unchecked().degrade(),
            29 => rp.PIN_29.clone_unchecked().degrade(),
            _ => panic!("Invalid pin number"),
        }
    }
}
