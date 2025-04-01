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
