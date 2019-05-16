
use std::process;

use raytracer;

fn wrapped_main() -> i32 {
    raytracer::main()
}

fn main() {
    let exit_code = wrapped_main();
    process::exit(exit_code);
}
