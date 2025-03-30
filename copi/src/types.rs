use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PostGpioOutputInitReq {
    pub rid: u16,
    pub pin: u8,
    pub value: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostGpioOutputSetReq {
    pub rid: u16,
    pub pin: u8,
    pub state: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPwmInitReq {
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
    pub pin: u8,
    pub percent: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPioLoadProgramReq {
    #[serde(rename = "pioNum")]
    pub pio_num: u8,
    pub program: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPioSmInitReq {
    #[serde(rename = "pioNum")]
    pub pio_num: u8,
    #[serde(rename = "smNum")]
    pub sm_num: u8,
    #[serde(rename = "pinNum")]
    pub pin_num: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPioSmSetEnabledReq {
    #[serde(rename = "pioNum")]
    pub pio_num: u8,
    #[serde(rename = "smNum")]
    pub sm_num: u8,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPioSmPushReq {
    #[serde(rename = "pioNum")]
    pub pio_num: u8,
    #[serde(rename = "smNum")]
    pub sm_num: u8,
    pub instr: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPioSmExecInstrReq {
    #[serde(rename = "pioNum")]
    pub pio_num: u8,
    #[serde(rename = "smNum")]
    pub sm_num: u8,
    #[serde(rename = "execInstr")]
    pub exec_instr: u16,
}
