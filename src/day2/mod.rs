use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::cmp;


pub fn run(part: u32) {
    let input = load("inputs/day2.txt").unwrap();
    let rows = parse(&input);

    if part == 1 {
        println!("Part 1");
        let answer = checksum(&rows);
        println!("{:?}", answer);
    }
    else {
        let answer = checksum2(&rows);
        println!("Part 2");
        println!("{:?}", answer);
    }
}

fn checksum2(rows: &Vec<Vec<usize>>) -> usize {
    let vals: Vec<usize> = 
        rows
        .iter()
        .map(|r| divisor_pair(r).unwrap() )
        .map(|(min, max)| max / min)
        .collect();
    vals.iter().fold(0, |acc: usize, x| x + acc)
}

fn checksum(rows: &Vec<Vec<usize>>) -> usize {
    let vals: Vec<usize> = 
        rows
        .iter()
        .map(|r| min_max(r) )
        .map(|(min, max)| max - min)
        .collect();
    vals.iter().fold(0, |acc: usize, x| x + acc)
}

fn min_max( row: &Vec<usize> ) -> (&usize, &usize) {
   let min = row.iter().min().unwrap();
   let max = row.iter().max().unwrap();
   (min, max)
}


fn parse(s: &String) -> Vec<Vec<usize>> {
   let rows: Vec<&str> = 
       s
       .split("\n")
        .filter(|&s| {s != "" && s != "\n"})
       .collect();

   let values = 
        rows
        .iter()
       .map(|row| parse_row(row) )
       .collect();
    values
}

fn parse_row(r: &str) -> Vec<usize> {
    r
        .split("\t")
        .map(|s| { parse_int(s) })
        .collect()
}


fn parse_int(s: &str) -> usize {
   usize::from_str_radix(s, 10).unwrap()
}

fn load(file_name: &str ) -> Result<String, Box<Error>> {    
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn divisor_for( n: usize, v: &[usize] ) -> Option<(usize, usize)> {
    if v == [] {
        None
    }
    else {
        let head = v[0];
        let tail = &v[1..];

        if (head % n == 0) || (n % head == 0) {
            Some((cmp::min(head,n),cmp::max(head,n)))
        } else {
            divisor_for(n, tail)
        }
    }
}

fn divisor_pair(v: &[usize] ) -> Option<(usize, usize)> {
   if v == [] {
       None
   } else {
       let head = v[0];
       let tail = &v[1..];
       match divisor_for(head, tail) {
           None => divisor_pair(tail),
           Some(pair) => Some(pair),
       }
   }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dont_find_divisor_for_empty_vec() {
        let v = vec![];
        let n = 5;
        let result = divisor_for(n, &v);
        assert!(result == None);
    }

    #[test]
    fn dont_find_divisor_for_nonempty_vec() {
        let v = vec![9,2,8];
        let n = 5;
        let result = divisor_for(n, &v);
        assert!(result == None);
    }

    #[test]
    fn find_divisor_for() {
        let v = vec![9,5,8];
        let n = 2;
        let result = divisor_for(n, &v);
        assert!(result == Some((2,8)));
    }

    #[test]
    fn no_divisor_pair() {
        let v = vec![];
        let result = divisor_pair(&v);
        assert!(result == None);
    }


} /* tests */
