use std::{env::var, fs::read_to_string, io::Result};

pub fn load_input_fallback(fallback: String) -> String {
    match load_input() {
        Ok(it) => it,
        Err(_) => {
            println!( "Failed to find input file at, falling back to test data");
            fallback
        }
    }
}

pub fn load_input() -> Result<String> {
    let root = match var("CARGO_MANIFEST_DIR") {
        Ok(it) => it,
        Err(_) => String::from(".."),
    };

    let path = format!("{}/resources/input", root);
    read_to_string(&path)
}