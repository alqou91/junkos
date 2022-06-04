extern crate cc;

use cc::Build;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut build = Build::new();
	build.file("src/trap.S").compile("trap");
	build.file("src/start.S").compile("start");
    Ok(())
}
