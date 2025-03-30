use std::collections::HashMap;

use axum::{Json, extract::State};
use copi_protocol::Command;
use pio_core::{PioVersion, ProgramWithDefines};
use pio_parser::Parser as PioParser;

use crate::{AppState, types::*};

use super::send_command;

pub async fn load_program(State(state): State<AppState>, Json(req): Json<PostPioLoadProgramReq>) {
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

    send_command(&mut port, pio_cmd);
}

pub async fn sm_init(State(state): State<AppState>, Json(req): Json<PostPioSmInitReq>) {
    let mut port = state.port.lock().unwrap();
    let pio_cmd = Command::PioSmInit {
        rid: 1,
        pio_num: req.pio_num,
        sm_num: req.sm_num,
        pin_num: req.pin_num,
    };
    send_command(&mut port, pio_cmd);
}

pub async fn sm_set_enabled(
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

pub async fn sm_push(State(state): State<AppState>, Json(req): Json<PostPioSmPushReq>) {
    let mut port = state.port.lock().unwrap();
    let pio_cmd = Command::PioSmPush {
        rid: 1,
        pio_num: req.pio_num,
        sm_num: req.sm_num,
        instr: req.instr,
    };
    send_command(&mut port, pio_cmd);
}

pub async fn sm_exec_instr(State(state): State<AppState>, Json(req): Json<PostPioSmExecInstrReq>) {
    let mut port = state.port.lock().unwrap();
    let pio_cmd = Command::PioSmExecInstr {
        rid: 1,
        pio_num: req.pio_num,
        sm_num: req.sm_num,
        exec_instr: req.exec_instr,
    };
    send_command(&mut port, pio_cmd);
}
