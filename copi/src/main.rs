use std::{
    collections::HashMap,
    io::Write,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{Json, Router, extract::State, routing::post};
use copi_protocol::Command;
use pio_core::{PioVersion, ProgramWithDefines};
use pio_parser::Parser as PioParser;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
use tokio_serial::{SerialPortBuilderExt as _, SerialStream};

const MAX_USB_PACKET_SIZE: usize = 64;

#[derive(Clone)]
pub struct AppState {
    port: Arc<Mutex<SerialStream>>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut port = tokio_serial::new("/dev/tty.usbmodem123456781", 0)
        .open_native_async()
        .unwrap();

    #[cfg(unix)]
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    let state = AppState {
        port: Arc::new(Mutex::new(port)),
    };

    let app = Router::new()
        .route("/gpio/output-init", post(post_gpio_output_init))
        .route("/gpio/output-set", post(post_gpio_output_set))
        .route("/pwm/init", post(post_init))
        .route(
            "/pwm/set-duty-cycle-percent",
            post(post_pwm_set_duty_cycle_percent),
        )
        .route("/pio/load_program", post(post_pio_load_program))
        .route("/pio/sm_init", post(post_pio_sm_init))
        .route("/pio/sm_set_enabled", post(post_pio_sm_set_enabled))
        .route("/pio/sm_push", post(post_pio_sm_push))
        .route("/pio/sm_exec_instr", post(post_pio_sm_exec_instr))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8899").await.unwrap();
    log::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostGpioOutputInitReq {
    rid: u16,
    pin: u8,
    value: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostGpioOutputInitRes {
    ok: bool,
}

#[axum::debug_handler]
pub async fn post_gpio_output_init(
    State(state): State<AppState>,
    Json(req): Json<PostGpioOutputInitReq>,
) -> Json<PostGpioOutputInitRes> {
    let mut port = state.port.lock().unwrap();

    let gpio_cmd = Command::GpioOutputInit {
        rid: req.rid,
        pin: req.pin,
        value: req.value,
    };

    let mut buf = [0u8; MAX_USB_PACKET_SIZE];
    let len = minicbor::len(&gpio_cmd);
    minicbor::encode(&gpio_cmd, buf.as_mut()).unwrap();

    // TODO should be async?
    let res = match port.write_all(&buf[..len]) {
        Ok(_) => {
            log::info!("Sent GPIO command: {:?}", gpio_cmd);
            true
        }
        Err(e) => {
            log::error!("Failed to send GPIO command: {:?}", e);
            false
        }
    };

    Json(PostGpioOutputInitRes { ok: res })
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostGpioOutputSetReq {
    rid: u16,
    pin: u8,
    state: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostGpioOutputSetRes {
    ok: bool,
}

#[axum::debug_handler]
pub async fn post_gpio_output_set(
    State(state): State<AppState>,
    Json(req): Json<PostGpioOutputSetReq>,
) -> Json<PostGpioOutputSetRes> {
    let mut port = state.port.lock().unwrap();

    let gpio_cmd = Command::GpioOutputSet {
        rid: req.rid,
        pin: req.pin,
        state: req.state,
    };

    let mut buf = [0u8; MAX_USB_PACKET_SIZE];
    let len = minicbor::len(&gpio_cmd);
    minicbor::encode(&gpio_cmd, buf.as_mut()).unwrap();

    // TODO should be async?
    let res = match port.write_all(&buf[..len]) {
        Ok(_) => {
            log::info!("Sent GPIO command: {:?}", gpio_cmd);
            true
        }
        Err(e) => {
            log::error!("Failed to send GPIO command: {:?}", e);
            false
        }
    };

    Json(PostGpioOutputSetRes { ok: res })
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPwmInitReq {
    slice: u8,
    a: Option<u8>,
    b: Option<u8>,
    divider: u8,
    #[serde(rename = "compareA")]
    compare_a: u16,
    #[serde(rename = "compareB")]
    compare_b: u16,
    top: u16,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostPwmInitRes {
    ok: bool,
}

#[axum::debug_handler]
pub async fn post_init(
    State(state): State<AppState>,
    Json(req): Json<PostPwmInitReq>,
) -> Json<PostPwmInitRes> {
    let mut port = state.port.lock().unwrap();

    let pwm_cmd = Command::PwmInit {
        rid: 1,
        slice: req.slice,
        a: req.a,
        b: req.b,
        divider: req.divider,
        compare_a: req.compare_a,
        compare_b: req.compare_b,
        top: req.top,
    };

    let mut buf = [0u8; MAX_USB_PACKET_SIZE];
    let len = minicbor::len(&pwm_cmd);
    minicbor::encode(&pwm_cmd, buf.as_mut()).unwrap();

    // TODO should be async?
    let res = match port.write_all(&buf[..len]) {
        Ok(_) => {
            log::info!("Sent PWM command: {:?}", pwm_cmd);
            true
        }
        Err(e) => {
            log::error!("Failed to send PWM command: {:?}", e);
            false
        }
    };

    Json(PostPwmInitRes { ok: res })
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPwmSetDutyCyclePercentReq {
    pin: u8,
    percent: u8,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostPwmSetDutyCyclePercentRes {
    ok: bool,
}

#[axum::debug_handler]
pub async fn post_pwm_set_duty_cycle_percent(
    State(state): State<AppState>,
    Json(req): Json<PostPwmSetDutyCyclePercentReq>,
) -> Json<PostPwmSetDutyCyclePercentRes> {
    let mut port = state.port.lock().unwrap();

    let pwm_cmd = Command::PwmSetDutyCyclePercent {
        rid: 1,
        pin: req.pin,
        percent: req.percent,
    };

    let mut buf = [0u8; MAX_USB_PACKET_SIZE];
    let len = minicbor::len(&pwm_cmd);
    minicbor::encode(&pwm_cmd, buf.as_mut()).unwrap();

    // TODO should be async?
    let res = match port.write_all(&buf[..len]) {
        Ok(_) => {
            log::info!("Sent PWM command: {:?}", pwm_cmd);
            true
        }
        Err(e) => {
            log::error!("Failed to send PWM command: {:?}", e);
            false
        }
    };

    Json(PostPwmSetDutyCyclePercentRes { ok: res })
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPioLoadProgramReq {
    #[serde(rename = "pioNum")]
    pio_num: u8,
    program: String,
}

pub async fn post_pio_load_program(
    State(state): State<AppState>,
    Json(req): Json<PostPioLoadProgramReq>,
) {
    let mut port = state.port.lock().unwrap();

    log::info!("Loading PIO program: {}", req.program);
    let program_parsed: ProgramWithDefines<HashMap<String, i32>, 16> =
        PioParser::parse_program(&req.program).unwrap();
    log::info!("Parsed PIO program: {:?}", program_parsed.program.code);

    let mut program: [u8; 32] = [0; 32];
    for (i, &value) in program_parsed.program.code.iter().enumerate() {
        let bytes = value.to_le_bytes();
        program[i * 2] = bytes[0];
        program[i * 2 + 1] = bytes[1];
    }

    let pio_cmd = Command::PioLoadProgram {
        rid: 1,
        pio_num: req.pio_num,
        program,
        program_len: (program_parsed.program.code.len() * 2) as u8, // as u8 len
        origin: program_parsed.program.origin,
        wrap_source: program_parsed.program.wrap.source,
        wrap_target: program_parsed.program.wrap.target,
        side_set_opt: program_parsed.program.side_set.optional(),
        side_set_bits: program_parsed.program.side_set.bits(),
        side_set_pindirs: program_parsed.program.side_set.pindirs(),
        pio_version_v0: program_parsed.program.version == PioVersion::V0,
    };
    log::info!("PIO command: {:?}", pio_cmd);

    let mut buf = [0u8; MAX_USB_PACKET_SIZE];
    let len = minicbor::len(&pio_cmd);
    minicbor::encode(&pio_cmd, buf.as_mut()).unwrap();

    port.write_all(&buf[..len]).unwrap();
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPioSmInitReq {
    #[serde(rename = "pioNum")]
    pio_num: u8,
    #[serde(rename = "smNum")]
    sm_num: u8,
    #[serde(rename = "pinNum")]
    pin_num: u8,
}

pub async fn post_pio_sm_init(State(state): State<AppState>, Json(req): Json<PostPioSmInitReq>) {
    let mut port = state.port.lock().unwrap();
    let pio_cmd = Command::PioSmInit {
        rid: 1,
        pio_num: req.pio_num,
        sm_num: req.sm_num,
        pin_num: req.pin_num,
    };
    send_command(&mut port, pio_cmd);
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPioSmSetEnabledReq {
    #[serde(rename = "pioNum")]
    pio_num: u8,
    #[serde(rename = "smNum")]
    sm_num: u8,
    enabled: bool,
}

pub async fn post_pio_sm_set_enabled(
    State(state): State<AppState>,
    Json(req): Json<PostPioSmSetEnabledReq>,
) {
    let mut port = state.port.lock().unwrap();
    let pio_cmd = Command::PioSmSetEnable {
        rid: 1,
        pio_num: req.pio_num,
        sm_num: req.sm_num,
        enable: req.enabled,
    };
    send_command(&mut port, pio_cmd);
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPioSmPushReq {
    #[serde(rename = "pioNum")]
    pio_num: u8,
    #[serde(rename = "smNum")]
    sm_num: u8,
    instr: u32,
}

pub async fn post_pio_sm_push(State(state): State<AppState>, Json(req): Json<PostPioSmPushReq>) {
    let mut port = state.port.lock().unwrap();
    let pio_cmd = Command::PioSmPush {
        rid: 1,
        pio_num: req.pio_num,
        sm_num: req.sm_num,
        instr: req.instr,
    };
    send_command(&mut port, pio_cmd);
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPioSmExecInstrReq {
    #[serde(rename = "pioNum")]
    pio_num: u8,
    #[serde(rename = "smNum")]
    sm_num: u8,
    #[serde(rename = "execInstr")]
    exec_instr: u16,
}

pub async fn post_pio_sm_exec_instr(
    State(state): State<AppState>,
    Json(req): Json<PostPioSmExecInstrReq>,
) {
    let mut port = state.port.lock().unwrap();
    let pio_cmd = Command::PioSmExecInstr {
        rid: 1,
        pio_num: req.pio_num,
        sm_num: req.sm_num,
        exec_instr: req.exec_instr,
    };
    send_command(&mut port, pio_cmd);
}

fn send_command(port: &mut SerialStream, cmd: Command) {
    let mut buf = [0u8; MAX_USB_PACKET_SIZE];
    let len = minicbor::len(&cmd);
    minicbor::encode(&cmd, buf.as_mut()).unwrap();

    // TODO should be async?
    match port.write_all(&buf[..len]) {
        Ok(_) => {
            log::info!("Sent command: {:?}", cmd);
        }
        Err(e) => {
            log::error!("Failed to send command: {:?}", e);
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use pio_core::ProgramWithDefines;
    use pio_parser::Parser;

    #[test]
    fn test_pio_program() {
        let program = "
            .side_set 1 opt
                pull noblock    side 0
                mov x, osr
                mov y, isr
            countloop:
                jmp x!=y noset
                jmp skip        side 1
            noset:
                nop
            skip:
                jmp y-- countloop
                ";

        println!("Program: {}", program);
        let program_parsed: ProgramWithDefines<HashMap<String, i32>, 32> =
            Parser::parse_program(program).unwrap();
        println!("{:?}", program_parsed.program.code);
    }
}
