use std::{error::Error, fmt::Pointer, fs};


const VALUES: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn match_word_forward(s: &str) -> Option<(char, usize)> {
    for start in 0..s.len() {
        for val in VALUES {
            if s[start..].starts_with(val.0) {
                return Some((val.1, start))
            }
        }
    }
    None
}

fn match_word_reverse(s: &str) -> Option<(char, usize)> {
    for start in (0..s.len()).rev() {
        for val in VALUES {
            if s[start..].starts_with(val.0) {
                return Some((val.1, start))
            }
        }
    }
    None
}

fn first_number(s: &str) -> Option<char> {
    let (char, idx) = match match_word_forward(s) {
        Some(r) => r,
        None => (' ', usize::MAX)
    };
    for (i, c) in s.chars().enumerate() {
        if c.is_numeric() && i < idx {
            return Some(c);
        }
    }
    
    if char == ' ' {
        return None;
    }
    Some(char)
}

fn last_number(s: &str) -> Option<char> {
    let (char, idx) = match match_word_reverse(s) {
        Some(r) => r,
        None => (' ', 0)
    };
    for (i, c) in s.chars().rev().enumerate() {
        if c.is_numeric() && s.len() - i > idx {
            return Some(c);
        }
    }
    if char == ' ' {
        return None
    }
    Some(char)
}

fn main() -> Result<(), Box<dyn Error>> {
    // read the file into lines
    let contents = fs::read_to_string("input.txt")?;
    let mut total = 0;
    for line in contents.lines() {
        if line.len() == 0 {
            break;
        }
        let first = first_number(line).unwrap();
        let last = last_number(line).unwrap();
        let line_total: i32 = [first, last].iter().collect::<String>().parse()?;
        total += line_total;
    }
    println!("{}", total);
    Ok(())
}
