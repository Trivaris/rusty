use std::collections::{HashMap, HashSet};

use shared::load_input;

fn main() {
    let body = load_input().expect("Couldnt find Input");

    let lines = body.split(",");

    let mut counter: u128 = 0;

    lines.for_each(|line| {
        let nums: Vec<u64> = line
            .split("-")
            .map(|f| match f.trim().parse::<u64>() {
                Ok(it) => it,
                Err(why) => {
                    panic!("Failed to parse {} because {}", f, why)
                }
            })
            .collect();
        if nums.len() > 2 {
            panic!("More than 2 nums in range")
        }
        let start = nums.first().expect("No nums found");
        let end = nums.last().expect("No nums found");

        for i in *start..=*end {
            let str = i.to_string();

            if check_is_repeat(&str) {
                counter += i as u128;
            };
        }
    });
    println!("It: {}", counter)
}

fn check_is_repeat(s: &str) -> bool {
    let s_char_len = s.chars().count();
    if s_char_len < 2 { return false; }
    let mut map: HashMap<char, u32> = HashMap::new();

    s.chars().for_each(|char| {
        let i = map.get(&char).unwrap_or(&0) + 1;
        map.insert(char, i);
    });

    let set: HashSet<u32> = map.values().cloned().collect();
    if set.len() != 1 {
        for sub_len in 1..=(s_char_len / 2) {
            if s_char_len % sub_len != 0 { continue; }
            let first_sub_str: String = s.chars().take(sub_len).collect();
            let num_repeats = s_char_len / sub_len;
            let reconstructed: String = first_sub_str.repeat(num_repeats);
            if reconstructed == s { return true; }
        };
    };

    let set_len = map.keys().count();

    if set_len == 1 {
        return is_all_same_char(s);
    }

    let char_count = match set.iter().next() {
        Some(it) => {
            if it > &1 { set_len as u32 }
            else { return false; }
        },
        None => {
            return false;
        }
    };

    if s_char_len % (char_count as usize) != 0 {
        return false;
    };

    let first_sub_str = s.chars().take(char_count as usize).collect::<String>();
    let mut current_char_index = char_count as usize;
    println!("Found Candidate {} with substr {}", s, first_sub_str);
    while current_char_index < s_char_len {
        let next_sub_str = s
            .chars()
            .skip(current_char_index)
            .take(char_count as usize)
            .collect::<String>();

        if next_sub_str != first_sub_str {
            return false;
        }

        current_char_index += char_count as usize;
    }
    return true;
}

fn is_all_same_char(s: &str) -> bool {
    let mut chars = s.chars();
    
    let first_char = match chars.next() {
        Some(c) => c,
        None => return true,
    };

    chars.all(|c| c == first_char)
}