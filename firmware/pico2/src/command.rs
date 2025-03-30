use copi_protocol::Command::{self, *};
use defmt::info;
use pio::{ArrayVec, PioVersion, Program, SideSet, Wrap};

use crate::peripherals::PeripheralController;

pub fn handle_command<'d>(pc: &mut PeripheralController<'d>, command: Command) {
    match command {
        GpioOutputInit { rid, pin, value } => {
            info!("GpioOutputInit: {} {}", pin, value);
            pc.gpio_output_init(pin as _, value);
        }
        GpioOutputSet { rid, pin, state } => {
            info!("GpioOutputSet: {} {}", pin, state);
            pc.gpio_output_set(pin as _, state);
        }
        PwmInit {
            rid,
            slice,
            a,
            b,
            divider,
            compare_a,
            compare_b,
            top,
        } => {
            info!(
                "PwmInit: {} {} {} {} {} {} {}",
                slice,
                a.unwrap_or(0),
                b.unwrap_or(0),
                divider,
                compare_a,
                compare_b,
                top
            );
            pc.pwm_init(slice, a, b, divider, compare_a, compare_b, top);
        }
        PwmSetDutyCyclePercent { rid, pin, percent } => {
            info!("PwmSetDutyCyclePercent: {} {} {}", rid, pin, percent);
            pc.pwm_set_duty_cycle_percent(pin, percent);
        }
        PioLoadProgram {
            rid,
            pio_num,
            program,
            program_len,
            origin,
            wrap_source,
            wrap_target,
            side_set_opt,
            side_set_bits,
            side_set_pindirs,
            pio_version_v0,
        } => {
            let mut program_u16 = [0u16; 16];
            for i in 0..16 {
                program_u16[i] = u16::from_le_bytes([program[i * 2], program[i * 2 + 1]]);
            }
            let program_u16_len = program_len / 2;

            let mut code = ArrayVec::<u16, 16>::new();
            for i in &program_u16[..program_u16_len as _] {
                code.push(*i);
            }
            pc.pio_load_program(
                pio_num as _,
                Program {
                    code,
                    origin,
                    wrap: Wrap {
                        source: wrap_source,
                        target: wrap_target,
                    },
                    side_set: SideSet::new(side_set_opt, side_set_bits - 1, side_set_pindirs),
                    version: if pio_version_v0 {
                        PioVersion::V0
                    } else {
                        PioVersion::V1
                    },
                },
            );
        }
        PioSmInit {
            rid,
            pio_num,
            sm_num,
            pin_num,
        } => {
            info!("PioSmInit: {} {} {}", rid, pio_num, sm_num);
            pc.pio_sm_init(pio_num as _, sm_num as _, pin_num as _);
        }
        PioSmSetEnable {
            rid,
            pio_num,
            sm_num,
            enable,
        } => {
            info!("PioSmSetEnable: {} {} {}", rid, pio_num, enable);
            pc.pio_sm_set_enable(pio_num as _, sm_num as _, enable);
        }
        PioSmPush {
            rid,
            pio_num,
            sm_num,
            instr,
        } => {
            info!("PioPush: {} {} {}", rid, pio_num, instr);
            pc.pio_sm_push(pio_num as _, sm_num as _, instr);
        }
        PioSmExecInstr {
            rid,
            pio_num,
            sm_num,
            exec_instr,
        } => {
            info!(
                "PioExecInstr: {} {} {} {}",
                rid, pio_num, sm_num, exec_instr
            );
            unsafe {
                pc.pio_sm_exec_instr_unchecked(pio_num as _, sm_num as _, exec_instr);
            }
        }
        _ => {
            info!("Unknown command");
        }
    }
}
