#[derive(::defmt::Format)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct RequestBody<'a> {
    #[femtopb(oneof, tags = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])]
    pub message: ::core::option::Option<request_body::Message<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `RequestBody`.
pub mod request_body {
    #[derive(::defmt::Format)]
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum Message<'a> {
        #[femtopb(message, tag = 1)]
        GetCpuFrequency(super::GetCpuFrequency<'a>),
        #[femtopb(message, tag = 2)]
        GpioOutputInit(super::GpioOutputInit<'a>),
        #[femtopb(message, tag = 3)]
        GpioOutputSet(super::GpioOutputSet<'a>),
        #[femtopb(message, tag = 4)]
        GpioOutputGet(super::GpioOutputGet<'a>),
        #[femtopb(message, tag = 5)]
        PwmInit(super::PwmInit<'a>),
        #[femtopb(message, tag = 6)]
        PwmSetDutyCyclePercent(super::PwmSetDutyCyclePercent<'a>),
        #[femtopb(message, tag = 7)]
        PioLoadProgram(super::PioLoadProgram<'a>),
        #[femtopb(message, tag = 8)]
        PioSmInit(super::PioSmInit<'a>),
        #[femtopb(message, tag = 9)]
        PioSmSetEnable(super::PioSmSetEnable<'a>),
        #[femtopb(message, tag = 10)]
        PioSmPush(super::PioSmPush<'a>),
        #[femtopb(message, tag = 11)]
        PioSmExecInstr(super::PioSmExecInstr<'a>),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
#[derive(::defmt::Format)]
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct GetCpuFrequency<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(::defmt::Format)]
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct GpioOutputInit<'a> {
    #[femtopb(uint32, tag = 1)]
    pub pin: u32,
    #[femtopb(bool, tag = 2)]
    pub value: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(::defmt::Format)]
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct GpioOutputSet<'a> {
    #[femtopb(uint32, tag = 1)]
    pub pin: u32,
    #[femtopb(bool, tag = 2)]
    pub value: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(::defmt::Format)]
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct GpioOutputGet<'a> {
    #[femtopb(uint32, tag = 1)]
    pub pin: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(::defmt::Format)]
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct PwmInit<'a> {
    #[femtopb(uint32, tag = 1)]
    pub slice: u32,
    #[femtopb(uint32, optional, tag = 2)]
    pub a: ::core::option::Option<u32>,
    #[femtopb(uint32, optional, tag = 3)]
    pub b: ::core::option::Option<u32>,
    #[femtopb(uint32, tag = 4)]
    pub divider: u32,
    #[femtopb(uint32, tag = 5)]
    pub compare_a: u32,
    #[femtopb(uint32, tag = 6)]
    pub compare_b: u32,
    #[femtopb(uint32, tag = 7)]
    pub top: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(::defmt::Format)]
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct PwmSetDutyCyclePercent<'a> {
    #[femtopb(uint32, tag = 1)]
    pub pin: u32,
    #[femtopb(uint32, tag = 2)]
    pub percent: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(::defmt::Format)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct PioLoadProgram<'a> {
    #[femtopb(uint32, tag = 1)]
    pub pio_num: u32,
    #[femtopb(string, tag = 2)]
    pub program: &'a str,
    #[femtopb(uint32, tag = 3)]
    pub program_len: u32,
    #[femtopb(uint32, optional, tag = 4)]
    pub origin: ::core::option::Option<u32>,
    #[femtopb(uint32, tag = 5)]
    pub wrap_source: u32,
    #[femtopb(uint32, tag = 6)]
    pub wrap_target: u32,
    #[femtopb(bool, tag = 7)]
    pub side_set_opt: bool,
    #[femtopb(uint32, tag = 8)]
    pub side_set_bits: u32,
    #[femtopb(bool, tag = 9)]
    pub side_set_pindirs: bool,
    #[femtopb(bool, tag = 10)]
    pub pio_version_v0: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(::defmt::Format)]
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct PioSmInit<'a> {
    #[femtopb(uint32, tag = 1)]
    pub pio_num: u32,
    #[femtopb(uint32, tag = 2)]
    pub sm_num: u32,
    #[femtopb(uint32, tag = 3)]
    pub pin_num: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(::defmt::Format)]
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct PioSmSetEnable<'a> {
    #[femtopb(uint32, tag = 1)]
    pub pio_num: u32,
    #[femtopb(uint32, tag = 2)]
    pub sm_num: u32,
    #[femtopb(bool, tag = 3)]
    pub enable: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(::defmt::Format)]
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct PioSmPush<'a> {
    #[femtopb(uint32, tag = 1)]
    pub pio_num: u32,
    #[femtopb(uint32, tag = 2)]
    pub sm_num: u32,
    #[femtopb(uint32, tag = 3)]
    pub instr: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(::defmt::Format)]
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct PioSmExecInstr<'a> {
    #[femtopb(uint32, tag = 1)]
    pub pio_num: u32,
    #[femtopb(uint32, tag = 2)]
    pub sm_num: u32,
    #[femtopb(uint32, tag = 3)]
    pub exec_instr: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(::defmt::Format)]
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct ResponseBody<'a> {
    #[femtopb(oneof, tags = [1])]
    pub message: ::core::option::Option<response_body::Message<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `ResponseBody`.
pub mod response_body {
    #[derive(::defmt::Format)]
    #[derive(Clone, Copy, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum Message<'a> {
        #[femtopb(message, tag = 1)]
        Common(super::Common<'a>),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
#[derive(::defmt::Format)]
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct Common<'a> {
    #[femtopb(uint32, tag = 1)]
    pub error: u32,
    #[femtopb(uint64, tag = 2)]
    pub data: u64,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(::defmt::Format)]
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum ResponseCommonErrorCode {
    #[default]
    UnknownError = 0,
    WrongPinState = 1,
}
impl ResponseCommonErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::UnknownError => "UNKNOWN_ERROR",
            Self::WrongPinState => "WRONG_PIN_STATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_ERROR" => Some(Self::UnknownError),
            "WRONG_PIN_STATE" => Some(Self::WrongPinState),
            _ => None,
        }
    }
}
#[derive(::defmt::Format)]
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct CopiRequest<'a> {
    #[femtopb(uint32, tag = 1)]
    pub request_id: u32,
    #[femtopb(message, optional, tag = 2)]
    pub payload: ::core::option::Option<RequestBody<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(::defmt::Format)]
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct CopiResponse<'a> {
    #[femtopb(uint32, tag = 1)]
    pub request_id: u32,
    #[femtopb(message, optional, tag = 2)]
    pub payload: ::core::option::Option<ResponseBody<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
