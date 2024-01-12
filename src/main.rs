use num_derive::FromPrimitive;    
// use num_traits::FromPrimitive;
use std::env;
use std::process::exit;

mod um_load;
mod um_segments;

use um_load::*;
use um_segments::*;

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

        let a = ((instruction >> 6) & 0x7) as usize;
        let b = ((instruction >> 3) & 0x7) as usize;
        let c = (instruction & 0x7) as usize;

        match opcode {
            OpCode::CMOV => {
                if registers[c] != 0 {
                    registers[a] = registers[b];
                }
            }
            OpCode::SLOAD => {
                registers[a] = env.get(registers[b] as usize)[registers[c] as usize];
            }
            OpCode::SSTORE => {
                env.get(registers[a] as usize)[registers[b] as usize] = registers[c];
            }
            OpCode::ADD => {
                registers[a] = registers[b] + registers[c];
            }
            OpCode::MUL => {
                registers[a] = registers[b] * registers[c];
            }
            OpCode::DIV => {
                registers[a] = registers[b] / registers[c];
            }
            OpCode::NAND => {
                registers[a] = !(registers[b] & registers[c]);
            }
            OpCode::HALT => {
                return;
            }
            OpCode::ALLOC => {
                registers[a] = env.alloc(registers[c] as usize) as u32;
            }
            OpCode::FREE => {
                env.free(registers[c] as usize);
            }
            OpCode::INPUT => {
                todo!();
            }
            OpCode::OUTPUT => {
                todo!();
            }
            OpCode::LOADP => {
                let segment = env.get(registers[b] as usize).clone();
                env.replace(0, segment);
                program_counter = registers[c] as usize;
            }
            OpCode::LOADV => {
                let a = ((instruction >> 25) & 0x7) as usize;
                let value = instruction & 0x1ffffff;
                registers[a] = value;
            }
        }
    }
}
