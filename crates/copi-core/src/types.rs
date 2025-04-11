use copi_protocol::DeviceMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize)]
pub struct CommonResponse {
    error: u16,
    data: u64,
}

impl From<DeviceMessage> for CommonResponse {
    fn from(msg: DeviceMessage) -> Self {
        match msg {
            DeviceMessage::Common { error, data } => Self { error, data },
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostGpioOutputInitReq {
    #[serde(rename = "skipResponse")]
    #[serde(default)]
    pub skip_response: bool,
    pub pin: u8,
    pub value: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostGpioOutputSetReq {
    #[serde(rename = "skipResponse")]
    #[serde(default)]
    pub skip_response: bool,
    pub pin: u8,
    pub state: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPwmInitReq {
    #[serde(rename = "skipResponse")]
    #[serde(default)]
    pub skip_response: bool,
    pub slice: u8,
    pub a: Option<u8>,
    pub b: Option<u8>,
    pub divider: u8,
    #[serde(rename = "compareA")]
    pub compare_a: u16,
    #[serde(rename = "compareB")]
    pub compare_b: u16,
    pub top: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPwmSetDutyCyclePercentReq {
    #[serde(rename = "skipResponse")]
    #[serde(default)]
    pub skip_response: bool,
    pub pin: u8,
    pub percent: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPioLoadProgramReq {
    #[serde(rename = "skipResponse")]
    #[serde(default)]
    pub skip_response: bool,
    #[serde(rename = "pioNum")]
    pub pio_num: u8,
    pub program: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPioSmInitReq {
    #[serde(rename = "skipResponse")]
    #[serde(default)]
    pub skip_response: bool,
    #[serde(rename = "pioNum")]
    pub pio_num: u8,
    #[serde(rename = "smNum")]
    pub sm_num: u8,
    #[serde(rename = "pinNum")]
    pub pin_num: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPioSmSetEnabledReq {
    #[serde(rename = "skipResponse")]
    #[serde(default)]
    pub skip_response: bool,
    #[serde(rename = "pioNum")]
    pub pio_num: u8,
    #[serde(rename = "smNum")]
    pub sm_num: u8,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPioSmPushReq {
    #[serde(rename = "skipResponse")]
    #[serde(default)]
    pub skip_response: bool,
    #[serde(rename = "pioNum")]
    pub pio_num: u8,
    #[serde(rename = "smNum")]
    pub sm_num: u8,
    pub instr: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPioSmExecInstrReq {
    #[serde(rename = "skipResponse")]
    #[serde(default)]
    pub skip_response: bool,
    #[serde(rename = "pioNum")]
    pub pio_num: u8,
    #[serde(rename = "smNum")]
    pub sm_num: u8,
    #[serde(rename = "execInstr")]
    pub exec_instr: u16,
}
