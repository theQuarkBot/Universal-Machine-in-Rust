use num_derive::FromPrimitive;    
// use num_traits::FromPrimitive;
use std::env;
use std::process::exit;

mod um_load;
mod umsegments;

use um_load::*;
use umsegments::*;

const NUM_REGISTERS: usize = 8;

#[derive(FromPrimitive)]
enum OpCode {
    CMOV = 0,
    SLOAD,
    SSTORE,
    ADD,
    MUL,
    DIV,
    NAND,
    HALT,
    ALLOC,
    FREE,
    OUTPUT,
    INPUT,
    LOADP,
    LOADV,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        exit(1);
    }

    um_run(read_program(&args[1]));
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
fn um_run(program: Vec<u32>) {
    let mut env = UmSegments::new_with_program(program);
    let mut registers: [u32; NUM_REGISTERS] = [0; NUM_REGISTERS];
    let mut program_counter: usize = 0;

    let mut seg_zero = env.get(0);

    loop {
        let instruction = seg_zero[program_counter];
        program_counter += 1;
        let opcode: OpCode = num::FromPrimitive::from_u32(instruction >> 28).unwrap();

        match opcode {
            OpCode::CMOV => {
                todo!();
            }
            OpCode::SLOAD => {
                todo!();
            }
            OpCode::SSTORE => {
                todo!();
            }
            OpCode::ADD => {
                todo!();
            }
            OpCode::MUL => {
                todo!();
            }
            OpCode::DIV => {
                todo!();
            }
            OpCode::NAND => {
                todo!();
            }
            OpCode::HALT => {
                todo!();
            }
            OpCode::ALLOC => {
                todo!();
            }
            OpCode::FREE => {
                todo!();
            }
            OpCode::INPUT => {
                todo!();
            }
            OpCode::OUTPUT => {
                todo!();
            }
            OpCode::LOADP => {
                todo!();
            }
            OpCode::LOADV => {
                todo!();
            }
        }
    }
}
