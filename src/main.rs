
use std::{env, process};

use raytracer;

fn wrapped_main() -> i32 {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 3 {
        eprintln!("Error: incorrect number of arguments");
        eprintln!("Usage: {} <scene file name> <output file name>", args[0]);
        return 1;
    }

    let scene_file_name = &args[1];
    let output_file_name = &args[2];

    // Call into lib.rs
    raytracer::main(scene_file_name, output_file_name)
}

fn main() {
    // Wrap main to simplify returning an exit code
    let exit_code = wrapped_main();
    process::exit(exit_code);
}
