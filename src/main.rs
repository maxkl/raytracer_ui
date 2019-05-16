
use std::{env, process};

use raytracer;

fn wrapped_main() -> i32 {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("Error: incorrect number of arguments");
        eprintln!("Usage: {} <output file name>", args[0]);
        return 1;
    }

    let output_file_name = &args[1];

    raytracer::main(output_file_name)
}

fn main() {
    let exit_code = wrapped_main();
    process::exit(exit_code);
}
