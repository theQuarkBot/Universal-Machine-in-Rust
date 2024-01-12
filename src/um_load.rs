use std::{fs, io::{BufReader, Read}};

pub fn read_program(filename: & str) -> Vec<u32> {
    let (file_size, mut reader) = open_file(filename);

    let mut program: Vec<u32> = Vec::with_capacity(file_size as usize / 4);
    let mut buffer: [u8; 4] = [0; 4];
    
    for _ in 0..file_size / 4 {
        reader.read_exact(&mut buffer).expect("Unable to read file");
        program.push(u32::from_be_bytes(buffer));
    }

    program
}

fn open_file(filename: &str) -> (u64, BufReader<fs::File>) {
    let file = fs::File::open(filename).expect("Unable to open file");
    let file_size = fs::metadata(filename).expect("Unable to read metadata").len();
    let reader = BufReader::new(file);

    assert!(file_size % 4 == 0);
    (file_size, reader)
}