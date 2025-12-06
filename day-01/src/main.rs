use std::{env::var, fs::read_to_string};

fn main() {
    let root = match var("CARGO_MANIFEST_DIR") {
        Ok(it) => it,
        Err(_) => String::from(".."),
    };

    let path = format!("{}/resources/input", root);
    let body = match read_to_string(&path) {
        Ok(it) => it,
        Err(_) => {
            println!(
                "Failed to find input file at {}, falling back to test data",
                &path
            );
            String::from("L13")
        }
    };

    let mut click_counter: i32 = 0;
    let mut cur_rotation: i32 = 50;

    for line in body.lines() {
        let (mut rotation, minus) = extract_line(line);
        click_counter += rotation / 100;
        rotation %= 100;

        let new_rotation = if minus {
            cur_rotation - rotation
        } else {
            cur_rotation + rotation
        };

        let corrected_pos = if minus {
            if new_rotation >= 0 {
                new_rotation
            } else {
                if cur_rotation > 0 {
                    click_counter += 1;
                };
                100 + new_rotation
            }
        } else {
            match new_rotation {
                x if x <= 99 => new_rotation,
                x if x > 100 => {
                    click_counter += 1;
                    new_rotation - 100
                }
                100 => 0,
                _ => panic!(),
            }
        };
        cur_rotation = corrected_pos;

        if cur_rotation == 0 {
            click_counter += 1;
        }
    }

    println!("The Password is {}", click_counter);
}

fn extract_line(line: &str) -> (i32, bool) {
    let mut chars = line.chars();
    let prefix = chars.next().expect("String empty.");
    let len = prefix.len_utf8();
    (line[len..].parse::<i32>().unwrap(), prefix == 'L')
}
