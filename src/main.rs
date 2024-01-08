use std::array;
use std::env;
use std::fs;
use std::io::BufReader;
use std::io::Read;
use std::process::exit;
use console::Term;
extern crate num;
#[macro_use]
extern crate num_derive;


const NUM_REGISTERS: usize = 8;
const INITIAL_NUM_SEGMENTS: usize = 100;

struct UmSegments {
    segments: Vec<Vec<u32>>,
    free_list: Vec<usize>,
}

#[derive(FromPrimitive)]
enum OpCode {
    CMOV,
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
    // --1. Check arguments and open given file
    // --2. Read file (the program) into vector of u32 words
    // 3. Create environment (registers, memory), and insert program array into segment zero
    // 4. Execute command loop
        // 4.1 Fetch next instruction
        // 4.2 Decode instruction
        // 4.3 Execute instruction
        // 4.4 Repeat until halt

    // Notes: Each segment's size cannot change, but the number of segments can
    //        The program counter is a register, and is initialized to zero
    //        The program counter is incremented after each instruction is executed
    //        The program itself is stored in segment zero

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        exit(1);
    }

    um_run(read_program(&args[1]));
}

fn read_program(filename: & str) -> Vec<u32> {
    let file = fs::File::open(filename).expect("Unable to open file");
    let file_size = fs::metadata(filename).expect("Unable to read metadata").len();
    let mut reader = BufReader::new(file);

    assert!(file_size % 4 == 0);

    let mut program: Vec<u32> = Vec::with_capacity(file_size as usize / 4);
    let mut buffer: [u8; 4] = [0; 4];
    
    for _ in 0..file_size / 4 {
        reader.read_exact(&mut buffer).expect("Unable to read file");
        program.push(u32::from_be_bytes(buffer));
    }

    program
}

fn um_run(program: Vec<u32>) {
    let mut env = init_env(program);
    let mut registers: [u32; NUM_REGISTERS] = [0; NUM_REGISTERS];
    let mut program_counter: u32 = 0;

    let mut term = Term::stdout();

    loop {
        let instruction = env.segments[0][program_counter as usize];
        program_counter += 1;
        let opcode: OpCode = num::FromPrimitive::from_u32(instruction >> 28).unwrap();

        match opcode {
            OpCode::CMOV => {
                let a = ((instruction >> 6) & 0x7) as usize;
                let b = ((instruction >> 3) & 0x7) as usize;
                let c = ((instruction >> 0) & 0x7) as usize;

                if registers[c] != 0 {
                    registers[a] = registers[b];
                }
            },
            OpCode::SLOAD => {
                let a = ((instruction >> 6) & 0x7) as usize;
                let b = ((instruction >> 3) & 0x7) as usize;
                let c = ((instruction >> 0) & 0x7) as usize;

                registers[a] = env.segments[registers[b] as usize][registers[c] as usize];
            },
            OpCode::SSTORE => {
                let a = ((instruction >> 6) & 0x7) as usize;
                let b = ((instruction >> 3) & 0x7) as usize;
                let c = ((instruction >> 0) & 0x7) as usize;

                env.segments[registers[a] as usize][registers[b] as usize] = registers[c];
            },
            OpCode::ADD => {
                let a = ((instruction >> 6) & 0x7) as usize;
                let b = ((instruction >> 3) & 0x7) as usize;
                let c = ((instruction >> 0) & 0x7) as usize;

                registers[a] = registers[b].wrapping_add(registers[c]);
            },
            OpCode::MUL => {
                let a = ((instruction >> 6) & 0x7) as usize;
                let b = ((instruction >> 3) & 0x7) as usize;
                let c = ((instruction >> 0) & 0x7) as usize;

                registers[a] = registers[b].wrapping_mul(registers[c]);
            },
            OpCode::DIV => {
                let a = ((instruction >> 6) & 0x7) as usize;
                let b = ((instruction >> 3) & 0x7) as usize;
                let c = ((instruction >> 0) & 0x7) as usize;

                registers[a] = registers[b].wrapping_div(registers[c]);
            },
            OpCode::NAND => {
                let a = ((instruction >> 6) & 0x7) as usize;
                let b = ((instruction >> 3) & 0x7) as usize;
                let c = ((instruction >> 0) & 0x7) as usize;

                registers[a] = !(registers[b] & registers[c]);
            },
            OpCode::HALT => {
                return;
            },
            OpCode::ALLOC => {
                let b = ((instruction >> 3) & 0x7) as usize;
                let c = ((instruction >> 0) & 0x7) as usize;

                let new_segment_id = if env.free_list.len() > 0 {
                    env.free_list.pop().unwrap()
                } else {
                    env.segments.push(Vec::with_capacity(registers[c] as usize));
                    env.segments.len() - 1
                };

                env.segments[new_segment_id] = vec![0; registers[c] as usize];
                registers[b] = new_segment_id as u32;
            },
            OpCode::FREE => {
                let c = ((instruction >> 0) & 0x7) as usize;

                env.free_list.push(registers[c] as usize);
            },
            OpCode::OUTPUT => {
                let c = ((instruction >> 0) & 0x7) as usize;

                print!("{}", registers[c] as u8 as char)
            },
            OpCode::INPUT => {
                let c = ((instruction >> 0) & 0x7) as usize;

                let input: u32 = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u32)
                    .unwrap();

                registers[c] = input;
            },
            OpCode::LOADP => {
                let b = ((instruction >> 3) & 0x7) as usize;
                let c = ((instruction >> 0) & 0x7) as usize;

                let new_segment = env.segments[registers[b] as usize].clone();
                env.segments[0] = new_segment;
                program_counter = registers[c];
            },
            OpCode::LOADV => {
                let a = ((instruction >> 6) & 0x7) as usize;
                let val = ((instruction >> 0) & 0x7FFFFFF) as u32;

                registers[a] = val;
            },
        }
    }
}

fn init_env(program: Vec<u32>) -> UmSegments {
    let mut env = UmSegments {
        segments: Vec::with_capacity(INITIAL_NUM_SEGMENTS),
        free_list: Vec::with_capacity(INITIAL_NUM_SEGMENTS)
    };

    env.segments.push(program);

    env
}

// #define SEGMENT(N) ((N)==0 ? prog : Umsegment_of_id(amap, (N)))
// #define OP(N) ((Um_Opcode)((N)>>28))
// #define A(N) (((N) >> 6) & 0x7)
// #define B(N) (((N) >> 3) & 0x7)
// #define C(N) (((N) >> 0) & 0x7)
// #define RA (regs[A(instruction)])
// #define RB (regs[B(instruction)])
// #define RC (regs[C(instruction)])
