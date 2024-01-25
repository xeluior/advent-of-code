use std::error::Error;
use std::fs;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input/2.txt")?;
    let red_re = Regex::new(r"([0-9]+) red")?;
    let blue_re = Regex::new(r"([0-9]+) blue")?;
    let green_re = Regex::new(r"([0-9]+) green")?;
    let game_re = Regex::new(r"Game ([0-9]+):")?;
    let count: usize = content.lines().filter_map(|line| {
        if line.len() == 0 { return None; }
        let reds = red_re.captures_iter(line).any(|cap| cap[1].parse::<usize>().unwrap() > 12);
        let greens = green_re.captures_iter(line).any(|cap| cap[1].parse::<usize>().unwrap() > 13);
        let blues = blue_re.captures_iter(line).any(|cap| cap[1].parse::<usize>().unwrap() > 14);
        if reds || greens || blues { return None; }

        match game_re.captures(line) {
            Some(cap) => Some(cap[1].parse::<usize>().unwrap()),
            None => panic!("No game number")
        }
    }).sum();
    println!("{}", count);
    Ok(())
}
