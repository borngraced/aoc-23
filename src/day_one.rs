use std::char;

pub fn day_one() -> u32 {
    let puzzle_input = include_str!("puzzle_input");
    puzzle_input
        .lines()
        .into_iter()
        .map(|e| {
            let e = e.chars().into_iter().collect::<Vec<char>>();
            str::parse::<u32>(&format!(
                "{}{}",
                get_first_digit(&e, 0),
                get_last_digit(&e, e.len())
            ))
            .expect("Valid u32 value")
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>()
}

fn get_first_digit(chars: &Vec<char>, start: usize) -> u32 {
    if chars.len() <= start {
        return 0;
    }

    if let Ok(n) = str::parse::<u32>(&chars[start].to_string()) {
        return n;
    } else {
        get_first_digit(chars, start + 1)
    }
}

fn get_last_digit(chars: &Vec<char>, start: usize) -> u32 {
    if start < 1 || start > chars.len() {
        return 0;
    }

    if let Ok(n) = str::parse::<u32>(&chars[start - 1].to_string()) {
        return n;
    } else {
        get_last_digit(chars, start - 1)
    }
}

