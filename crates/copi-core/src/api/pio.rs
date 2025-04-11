use std::collections::HashMap;

use axum::{Json, extract::State, http::StatusCode};
use copi_protocol::HostMessage;
use pio_core::{PioVersion, ProgramWithDefines};
use pio_parser::Parser as PioParser;

use crate::{AppState, process_common, types::*};

#[axum::debug_handler]
pub async fn load_program(
    State(mut state): State<AppState>,
    Json(req): Json<PostPioLoadProgramReq>,
) -> Result<Json<CommonResponse>, StatusCode> {
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

    let msg = HostMessage::PioLoadProgram {
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
    log::info!("PIO command: {:?}", msg);

    process_common!(state, req, msg)
}

#[axum::debug_handler]
pub async fn sm_init(
    State(mut state): State<AppState>,
    Json(req): Json<PostPioSmInitReq>,
) -> Result<Json<CommonResponse>, StatusCode> {
    let msg = HostMessage::PioSmInit {
        pio_num: req.pio_num,
        sm_num: req.sm_num,
        pin_num: req.pin_num,
    };
    process_common!(state, req, msg)
}

#[axum::debug_handler]
pub async fn sm_set_enabled(
    State(mut state): State<AppState>,
    Json(req): Json<PostPioSmSetEnabledReq>,
) -> Result<Json<CommonResponse>, StatusCode> {
    let msg = HostMessage::PioSmSetEnable {
        pio_num: req.pio_num,
        sm_num: req.sm_num,
        enable: req.enabled,
    };
    process_common!(state, req, msg)
}

#[axum::debug_handler]
pub async fn sm_push(
    State(mut state): State<AppState>,
    Json(req): Json<PostPioSmPushReq>,
) -> Result<Json<CommonResponse>, StatusCode> {
    let msg = HostMessage::PioSmPush {
        pio_num: req.pio_num,
        sm_num: req.sm_num,
        instr: req.instr,
    };
    process_common!(state, req, msg)
}

#[axum::debug_handler]
pub async fn sm_exec_instr(
    State(mut state): State<AppState>,
    Json(req): Json<PostPioSmExecInstrReq>,
) -> Result<Json<CommonResponse>, StatusCode> {
    let msg = HostMessage::PioSmExecInstr {
        pio_num: req.pio_num,
        sm_num: req.sm_num,
        exec_instr: req.exec_instr,
    };
    process_common!(state, req, msg)
}
