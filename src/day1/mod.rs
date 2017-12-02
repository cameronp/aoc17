use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


pub fn run(part: u32) {
    let answer = compute(part);
    println!("Week {}: ", part);
    println!("{}", answer);
}

fn compute(part: u32) -> u32 {
    let input = load("inputs/day1.txt").unwrap();
    let values: Vec<u8> = parse(input);

    if part == 1 {
        process(values)
    }
    else {
        process2(values)
    }
}

fn parse(s: String) -> Vec<u8> {
   s.split("")
    .filter(|&s| {s != "" && s != "\n"})
    .map(|s| { parse_int(s) })
    .collect()
}

fn parse_int(s: &str) -> u8 {
   u8::from_str_radix(s, 10).unwrap()
}

fn load(file_name: &str ) -> Result<String, Box<Error>> {    
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn process2(v: Vec<u8>) -> u32 {
    let size = v.len() ;
    let offset = size / 2;
    (0..size)
        .filter(|&u| v[u] == v[(u + offset) % size])
        .map(|i| v[i])
        .fold(0, |acc: u32, x| acc + (x as u32) )
}
    

fn process(v: Vec<u8>) -> u32 {
    let first = v.first().unwrap();
    let last = v.last().unwrap();
    let chunks: Vec<u8> = v
        .windows(2)
        .filter(|chunk| { is_valid_chunk(chunk) })
        .map(|chunk| { chunk[0] })
        .collect();

    let linear_result: u32 = chunks
        .iter()
        .fold(0, |acc: u32, &x| acc + (x as u32) );

    if first == last {
        linear_result + (*first as u32)
    }
    else {
        linear_result
    }
}

fn is_valid_chunk(chunk: &[u8]) -> bool {
    chunk[0] == chunk[1]
}

#[cfg(test)]    
mod test {
    use super::*;

    #[test]
    fn part1() {
        let result = compute(1);
        assert!(result == 1341);
    }

    #[test]
    fn part2() {
        let result = compute(2);
        assert!(result == 1348);
    }
} /* test */
