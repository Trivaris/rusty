use std::{fs::read_to_string, path::PathBuf};

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = root.join("input").join("input");

    let mut count = 0;
    let mut rot: i16 = 50;
    for line in read_to_string(path).unwrap().lines() {
        let num = split_line_to_num(line);
        let res = (rot+num).rem_euclid(100);
        println!("Calculating {}+({})={}, from {}", rot, num, res, line);
        if part_two(&num, &res, &rot) { count+=1; };
        rot = res;
    };

    println!("The Password is {}", count);
}

fn split_line_to_num(line: &str) -> i16 {
    let mut chars = line.chars();
    let prefix = chars.next().expect("String empty.");
    let len = prefix.len_utf8();
    let minus = prefix == 'L';
    let rest = &line[len..].parse::<i16>().unwrap();
    if minus { -1*rest } else { *rest }
}

fn part_one(res: &i16) -> bool {
    *res == 0
}

fn part_two(num: &i16, res: &i16, rot: &i16 ) -> bool {
    (rot+num >= 100) || (0 >= rot+num)
}
