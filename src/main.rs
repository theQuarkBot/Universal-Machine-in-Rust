use std::env;
use std::process::exit;

mod um_load;
mod um_segments;
mod um;

use um_load::read_program;
use um::um_run;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        exit(1);
    }

    um_run(read_program(&args[1]));
}
