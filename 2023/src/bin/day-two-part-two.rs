use std::error::Error;
use std::fs;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input/2.txt")?;
    let red_re = Regex::new(r"([0-9]+) red")?;
    let blue_re = Regex::new(r"([0-9]+) blue")?;
    let green_re = Regex::new(r"([0-9]+) green")?;
    let game_re = Regex::new(r"Game ([0-9]+):")?;
    let count: usize = content.lines().map(|line| {
        if line.len() == 0 { return 0; }
        let reds = red_re.captures_iter(line).map(|cap| cap[1].parse::<usize>().unwrap()).max().unwrap_or(0);
        let greens = green_re.captures_iter(line).map(|cap| cap[1].parse::<usize>().unwrap()).max().unwrap_or(0);
        let blues = blue_re.captures_iter(line).map(|cap| cap[1].parse::<usize>().unwrap()).max().unwrap_or(0);

        reds * greens * blues
    }).sum();
    println!("{}", count);
    Ok(())
}
