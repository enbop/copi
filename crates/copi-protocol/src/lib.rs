#![cfg_attr(target_abi = "eabi", no_std)]
#![cfg_attr(target_abi = "eabihf", no_std)]

use minicbor::{CborLen, Decode, Encode};

#[derive(Debug, Encode, Decode, CborLen, PartialEq)]
pub enum Command {
    #[n(0)]
    Version {
        #[n(1)]
        major: u16,
        #[n(2)]
        minor: u16,
        #[n(3)]
        patch: u16,
    },
    #[n(1)]
    GetCpuFrequency {
        #[n(0)]
        rid: u16,
        #[n(1)]
        freq: u32,
    },
    #[n(64)]
    GpioOutputInit {
        #[n(0)]
        rid: u16,
        #[n(1)]
        pin: u8,
        #[n(2)]
        value: bool,
    },
    #[n(65)]
    GpioOutputSet {
        #[n(0)]
        rid: u16,
        #[n(1)]
        pin: u8,
        #[n(2)]
        state: bool,
    },
    #[n(66)]
    GpioOutputGet {
        #[n(0)]
        rid: u16,
        #[n(1)]
        pin: u8,
    },
    #[n(67)]
    PwmInit {
        #[n(0)]
        rid: u16,
        #[n(1)]
        slice: u8,
        #[n(2)]
        a: Option<u8>,
        #[n(3)]
        b: Option<u8>,
        #[n(4)]
        divider: u8,
        #[n(5)]
        compare_a: u16,
        #[n(6)]
        compare_b: u16,
        #[n(7)]
        top: u16,
    },
    #[n(68)]
    PwmSetDutyCyclePercent {
        #[n(0)]
        rid: u16,
        #[n(1)]
        pin: u8,
        #[n(2)]
        percent: u8,
    },
    #[n(69)]
    PioLoadProgram {
        #[n(0)]
        rid: u16,
        #[n(1)]
        pio_num: u8,
        // TODO currently only supports 32 byte programs (16 instructions)
        #[cbor(n(2), with = "minicbor::bytes")]
        program: [u8; 32],
        #[n(3)]
        program_len: u8,
        #[n(4)]
        origin: Option<u8>,
        #[n(5)]
        wrap_source: u8,
        #[n(6)]
        wrap_target: u8,
        #[n(7)]
        side_set_opt: bool,
        #[n(8)]
        side_set_bits: u8,
        #[n(9)]
        side_set_pindirs: bool,
        #[n(10)]
        pio_version_v0: bool,
    },
    #[n(70)]
    PioSmInit {
        #[n(0)]
        rid: u16,
        #[n(1)]
        pio_num: u8,
        #[n(2)]
        sm_num: u8,
        #[n(3)]
        pin_num: u8,
    },
    #[n(71)]
    PioSmSetEnable {
        #[n(0)]
        rid: u16,
        #[n(1)]
        pio_num: u8,
        #[n(2)]
        sm_num: u8,
        #[n(3)]
        enable: bool,
    },
    #[n(72)]
    PioSmPush {
        #[n(0)]
        rid: u16,
        #[n(1)]
        pio_num: u8,
        #[n(2)]
        sm_num: u8,
        #[n(3)]
        instr: u32,
    },
    #[n(73)]
    PioSmExecInstr {
        #[n(0)]
        rid: u16,
        #[n(1)]
        pio_num: u8,
        #[n(2)]
        sm_num: u8,
        #[n(3)]
        exec_instr: u16,
    },
}

#[cfg(test)]
mod tests {
    use core::u8;

    use super::*;

    #[test]
    fn vaild_command_len() {
        const MAX_LEN: usize = 64;
        assert!(
            minicbor::len(&Command::PwmInit {
                rid: u16::MAX,
                slice: u8::MAX,
                a: Some(u8::MAX),
                b: Some(u8::MAX),
                divider: u8::MAX,
                compare_a: u16::MAX,
                compare_b: u16::MAX,
                top: u16::MAX,
            }) < MAX_LEN
        );
    }

    #[test]
    fn test_command_codec() {
        let mut buf = [0u8; 64];

        let gpio_cmd = Command::GpioOutputSet {
            rid: 0,
            pin: 1,
            state: true,
        };
        let len = minicbor::len(&gpio_cmd);
        minicbor::encode(&gpio_cmd, buf.as_mut()).unwrap();
        println!("Encoded data: {:?}", &buf[..len]);

        let encoded_data = &buf[..len].to_owned();
        let decoded_cmd: Command = minicbor::decode(&encoded_data).unwrap();
        println!("Decoded command: {:?}", decoded_cmd);
        assert_eq!(decoded_cmd, gpio_cmd);
    }
}
