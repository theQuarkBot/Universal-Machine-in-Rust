use std::io::Read;

use num_derive::FromPrimitive;
// use num_traits::FromPrimitive;

// mod um_segments;

use crate::um_segments::*;

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

impl OpCode {
    fn from_u32(n: u32) -> Option<OpCode> {
        num::FromPrimitive::from_u32(n)
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn um_run(program: Vec<u32>) {
    let mut env = UmSegments::new_with_program(program);
    let mut registers: [u32; NUM_REGISTERS] = [0_u32; NUM_REGISTERS];
    let mut program_counter: usize = 0;

    // let seg_zero = env.get(0);

    loop {
        let instruction = env.get(0)[program_counter];
        program_counter += 1;
        
        let opcode: OpCode = OpCode::from_u32(instruction >> 28).unwrap();

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
                print!("LOADING:");
                registers[a] = env.get(registers[b] as usize)[registers[c] as usize];
                print!("{}", registers[a] as u8 as char);
                print!("{}", env.get(registers[b] as usize)[registers[c] as usize] as u8);
            }
            OpCode::SSTORE => {
                env.get(registers[a] as usize)[registers[b] as usize] = registers[c];
            }
            OpCode::ADD => {
                registers[a] = registers[b].overflowing_add(registers[c]).0;
            }
            OpCode::MUL => {
                registers[a] = registers[b].overflowing_mul(registers[c]).0;
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
                let mut buf: [u8; 1] = [0_u8; 1];
                std::io::stdin().read_exact(&mut buf).unwrap();
                registers[c] = buf[0] as u32;
            }
            OpCode::OUTPUT => {
                print!("{}", registers[c] as u8 as char);
            }
            OpCode::LOADP => {
                if registers[b] != 0 {
                    let segment = env.get(registers[b] as usize).clone();
                    env.replace(0, segment);
                }
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
