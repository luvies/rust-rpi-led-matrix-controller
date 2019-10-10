use std::path::Path;
use std::process::Command;
use std::env

fn main() {
    let matrix_lib = Path::new("vendor/rpi-rgb-led-matrix");

    Command::new("make")
        .current_dir(&matrix_lib)
        .status()
        .unwrap();

    println!(r"cargo:rustc-link-search={}/vendor/rpi-rgb-led-matrix/lib", env::current_dir().unwrap());
}
