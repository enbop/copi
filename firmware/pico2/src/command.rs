use defmt::info;
use pio::{ArrayVec, PioVersion, Program, SideSet, Wrap};

use crate::{generated::copi::*, peripherals::PeripheralController};

pub fn handle_request<'d>(
    pc: &'d mut PeripheralController<'static>,
    request: CopiRequest,
) -> CopiResponse<'d> {
    let request_id = request.request_id;
    let Some(payload) = request.payload else {
        return CopiResponse::default();
    };
    let Some(message) = payload.message else {
        return CopiResponse::default();
    };
    use request_body::Message;
    let response_body = match message {
        Message::GpioOutputInit(GpioOutputInit {
            pin,
            value,
            unknown_fields: _,
        }) => {
            info!("GpioOutputInit: {} {}", pin, value);
            pc.gpio_output_init(pin as _, value)
        }
        Message::GpioOutputSet(GpioOutputSet {
            pin,
            state,
            unknown_fields: _,
        }) => {
            info!("GpioOutputSet: {} {}", pin, state);
            pc.gpio_output_set(pin as _, state)
        }
        // TODO: Uncomment and implement these modules as needed
        // PwmInit {
        //     slice,
        //     a,
        //     b,
        //     divider,
        //     compare_a,
        //     compare_b,
        //     top,
        // } => {
        //     info!(
        //         "PwmInit: {} {} {} {} {} {} {}",
        //         slice,
        //         a.unwrap_or(0),
        //         b.unwrap_or(0),
        //         divider,
        //         compare_a,
        //         compare_b,
        //         top
        //     );
        //     pc.pwm_init(slice, a, b, divider, compare_a, compare_b, top)
        // }
        // PwmSetDutyCyclePercent { pin, percent } => {
        //     info!("PwmSetDutyCyclePercent: {} {}", pin, percent);
        //     pc.pwm_set_duty_cycle_percent(pin, percent)
        // }
        // PioLoadProgram {
        //     pio_num,
        //     program,
        //     program_len,
        //     origin,
        //     wrap_source,
        //     wrap_target,
        //     side_set_opt,
        //     side_set_bits,
        //     side_set_pindirs,
        //     pio_version_v0,
        // } => {
        //     let mut program_u16 = [0u16; 16];
        //     for i in 0..16 {
        //         program_u16[i] = u16::from_le_bytes([program[i * 2], program[i * 2 + 1]]);
        //     }
        //     let program_u16_len = program_len / 2;

        //     let mut code = ArrayVec::<u16, 16>::new();
        //     for i in &program_u16[..program_u16_len as _] {
        //         code.push(*i);
        //     }
        //     pc.pio_load_program(
        //         pio_num as _,
        //         Program {
        //             code,
        //             origin,
        //             wrap: Wrap {
        //                 source: wrap_source,
        //                 target: wrap_target,
        //             },
        //             side_set: SideSet::new(side_set_opt, side_set_bits - 1, side_set_pindirs),
        //             version: if pio_version_v0 {
        //                 PioVersion::V0
        //             } else {
        //                 PioVersion::V1
        //             },
        //         },
        //     )
        // }
        // PioSmInit {
        //     pio_num,
        //     sm_num,
        //     pin_num,
        // } => {
        //     info!("PioSmInit: {} {}", pio_num, sm_num);
        //     pc.pio_sm_init(pio_num as _, sm_num as _, pin_num as _)
        // }
        // PioSmSetEnable {
        //     pio_num,
        //     sm_num,
        //     enable,
        // } => {
        //     info!("PioSmSetEnable: {} {}", pio_num, enable);
        //     pc.pio_sm_set_enable(pio_num as _, sm_num as _, enable)
        // }
        // PioSmPush {
        //     pio_num,
        //     sm_num,
        //     instr,
        // } => {
        //     info!("PioPush: {} {}", pio_num, instr);
        //     pc.pio_sm_push(pio_num as _, sm_num as _, instr)
        // }
        // PioSmExecInstr {
        //     pio_num,
        //     sm_num,
        //     exec_instr,
        // } => {
        //     info!("PioExecInstr: {} {} {}", pio_num, sm_num, exec_instr);
        //     unsafe { pc.pio_sm_exec_instr_unchecked(pio_num as _, sm_num as _, exec_instr) }
        // }
        _ => {
            let message = response_body::Message::Common(Common {
                error: ResponseCommonErrorCode::UnknownError as _,
                data: 0,
                unknown_fields: Default::default(),
            });

            let mut res = ResponseBody::default();
            res.message.replace(message);
            res
        }
    };
    if request_id == 0 {
        return CopiResponse::default();
    }
    let mut response = CopiResponse::default();
    response.request_id = request_id;
    response.payload.replace(response_body);
    response
}
