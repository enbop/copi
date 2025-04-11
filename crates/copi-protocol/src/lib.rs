#![cfg_attr(target_abi = "eabi", no_std)]
#![cfg_attr(target_abi = "eabihf", no_std)]

use core::ops::Deref;

use minicbor::{CborLen, Decode, Encode};

#[derive(Debug, Encode, Decode, CborLen, PartialEq)]
pub struct CopiRequest {
    /// Request ID
    /// helper for the async response callback
    /// 0 means no response is expected
    #[n(0)]
    request_id: u16,
    #[n(1)]
    message: HostMessage,
}

impl CopiRequest {
    pub fn new(request_id: u16, command: HostMessage) -> Self {
        Self {
            request_id,
            message: command,
        }
    }

    pub fn new_without_id(command: HostMessage) -> Self {
        Self {
            request_id: 0,
            message: command,
        }
    }

    pub fn request_id(&self) -> u16 {
        self.request_id
    }

    pub fn into_message(self) -> HostMessage {
        self.message
    }
}

impl Deref for CopiRequest {
    type Target = HostMessage;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

#[derive(Debug, Encode, Decode, CborLen, PartialEq)]
pub struct CopiResponse {
    #[n(0)]
    request_id: u16,
    #[n(1)]
    message: DeviceMessage,
}

impl CopiResponse {
    pub fn new(request_id: u16, message: DeviceMessage) -> Self {
        Self {
            request_id,
            message,
        }
    }

    pub fn empty() -> Self {
        Self {
            request_id: 0,
            message: DeviceMessage::empty_ok(),
        }
    }

    pub fn request_id(&self) -> u16 {
        self.request_id
    }

    pub fn into_message(self) -> DeviceMessage {
        self.message
    }
}

impl Deref for CopiResponse {
    type Target = DeviceMessage;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

/// Message from Host to Device
#[derive(Debug, Encode, Decode, CborLen, PartialEq)]
pub enum HostMessage {
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
        freq: u32,
    },
    #[n(64)]
    GpioOutputInit {
        #[n(0)]
        pin: u8,
        #[n(1)]
        value: bool,
    },
    #[n(65)]
    GpioOutputSet {
        #[n(0)]
        pin: u8,
        #[n(1)]
        state: bool,
    },
    #[n(66)]
    GpioOutputGet {
        #[n(0)]
        pin: u8,
    },
    #[n(67)]
    PwmInit {
        #[n(0)]
        slice: u8,
        #[n(1)]
        a: Option<u8>,
        #[n(2)]
        b: Option<u8>,
        #[n(3)]
        divider: u8,
        #[n(4)]
        compare_a: u16,
        #[n(5)]
        compare_b: u16,
        #[n(6)]
        top: u16,
    },
    #[n(68)]
    PwmSetDutyCyclePercent {
        #[n(0)]
        pin: u8,
        #[n(1)]
        percent: u8,
    },
    #[n(69)]
    PioLoadProgram {
        #[n(0)]
        pio_num: u8,
        // TODO currently only supports 32 byte programs (16 instructions)
        #[cbor(n(1), with = "minicbor::bytes")]
        program: [u8; 32],
        #[n(2)]
        program_len: u8,
        #[n(3)]
        origin: Option<u8>,
        #[n(4)]
        wrap_source: u8,
        #[n(5)]
        wrap_target: u8,
        #[n(6)]
        side_set_opt: bool,
        #[n(7)]
        side_set_bits: u8,
        #[n(8)]
        side_set_pindirs: bool,
        #[n(9)]
        pio_version_v0: bool,
    },
    #[n(70)]
    PioSmInit {
        #[n(0)]
        pio_num: u8,
        #[n(1)]
        sm_num: u8,
        #[n(2)]
        pin_num: u8,
    },
    #[n(71)]
    PioSmSetEnable {
        #[n(0)]
        pio_num: u8,
        #[n(1)]
        sm_num: u8,
        #[n(2)]
        enable: bool,
    },
    #[n(72)]
    PioSmPush {
        #[n(0)]
        pio_num: u8,
        #[n(1)]
        sm_num: u8,
        #[n(2)]
        instr: u32,
    },
    #[n(73)]
    PioSmExecInstr {
        #[n(0)]
        pio_num: u8,
        #[n(1)]
        sm_num: u8,
        #[n(2)]
        exec_instr: u16,
    },
}

/// Message from Device to Host
#[derive(Debug, Encode, Decode, CborLen, PartialEq)]
pub enum DeviceMessage {
    #[n(0)]
    Common {
        #[n(0)]
        error: u16,
        #[n(1)]
        data: u64,
    },
}

impl DeviceMessage {
    pub fn empty_ok() -> Self {
        Self::Common { error: 0, data: 0 }
    }

    pub fn common(data: u64) -> Self {
        Self::Common { error: 0, data }
    }

    pub fn unknown_error() -> Self {
        Self::Common {
            error: DeviceCommonErrorCode::UnknowError as _,
            data: 0,
        }
    }
}

#[repr(C)]
pub enum DeviceCommonErrorCode {
    UnknowError = 1,
    WrongPinState = 2,
}

#[cfg(test)]
mod tests {
    use core::u8;

    use super::*;

    #[test]
    fn vaild_command_len() {
        const MAX_LEN: usize = 64;
        let request = CopiRequest::new(
            0,
            HostMessage::GpioOutputSet {
                pin: 1,
                state: true,
            },
        );
        assert!(minicbor::len(&request) < MAX_LEN);
    }

    #[test]
    fn test_command_codec() {
        let mut buf = [0u8; 64];

        let gpio_cmd = HostMessage::GpioOutputSet {
            pin: 1,
            state: true,
        };
        let gpio_req = CopiRequest::new_without_id(gpio_cmd);
        let len = minicbor::len(&gpio_req);
        minicbor::encode(&gpio_req, buf.as_mut()).unwrap();
        println!("Encoded data: {:?}", &buf[..len]);

        let encoded_data = &buf[..len].to_owned();
        let decoded: CopiRequest = minicbor::decode(&encoded_data).unwrap();
        println!("Decoded: {:?}", decoded);
        assert_eq!(decoded, gpio_req);
    }
}
