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
    SetGPIO {
        #[n(0)]
        rid: u16,
        #[n(1)]
        pin: u8,
        #[n(2)]
        state: bool,
    },
    #[n(65)]
    GetGPIO {
        #[n(0)]
        rid: u16,
        #[n(1)]
        pin: u8,
    },
    #[n(66)]
    SetPWM {
        #[n(0)]
        name: u8,
        #[n(1)]
        period: u32,
        #[n(2)]
        duty_cycle: u32,
        #[n(3)]
        percent: u8,
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
            minicbor::len(&Command::SetGPIO {
                rid: u16::MAX,
                pin: u8::MAX,
                state: true,
            }) < MAX_LEN
        );
        assert!(
            minicbor::len(&Command::SetPWM {
                name: u8::MAX,
                period: u32::MAX,
                duty_cycle: u32::MAX,
                percent: u8::MAX,
            }) < MAX_LEN
        );
    }

    #[test]
    fn test_command_codec() {
        let mut buf = [0u8; 64];

        let gpio_cmd = Command::SetGPIO {
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

        let pwm_cmd = Command::SetPWM {
            name: 1,
            period: 2,
            duty_cycle: 3,
            percent: 4,
        };
        let len = minicbor::len(&pwm_cmd);
        minicbor::encode(&pwm_cmd, buf.as_mut()).unwrap();
        println!("Encoded data: {:?}", &buf[..len]);

        let encoded_data = &buf[..len].to_owned();
        let decoded_cmd: Command = minicbor::decode(&encoded_data).unwrap();
        println!("Decoded command: {:?}", decoded_cmd);
        assert_eq!(decoded_cmd, pwm_cmd);
    }
}
