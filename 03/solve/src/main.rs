#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::io;

#[derive(Debug)]
#[derive(PartialEq)]
struct Claim {
    id: i32,
    start_row: usize,
    start_col: usize,
    num_rows: usize,
    num_cols: usize,
}

// Parses a claim from a string.
fn parse_claim(claim_str: &str) -> Option<Claim> {
    lazy_static! {
        static ref RE: Regex = Regex::new(concat!(
            r"#(?P<id>\d+) @ (?P<start_col>\d+),(?P<start_row>\d+): ",
            r"(?P<num_cols>\d+)x(?P<num_rows>\d+)")).unwrap();
    }
    // println!("re: {:?}", *RE);
    // println!("claim_str: {}", claim_str);
    // println!("caps: {:?}", RE.captures(claim_str));
    match RE.captures(claim_str) {
        Some(caps) => Some(Claim{
            id: caps.name("id").unwrap().as_str().parse().unwrap(),
            start_row: caps.name("start_row").unwrap().as_str().parse().unwrap(),
            start_col: caps.name("start_col").unwrap().as_str().parse().unwrap(),
            num_rows: caps.name("num_rows").unwrap().as_str().parse().unwrap(),
            num_cols: caps.name("num_cols").unwrap().as_str().parse().unwrap()}),
        None => None
    }
}

#[test]
fn test_parse_claim() {
    assert_eq!(parse_claim("#1 @ 2,3: 4x5"),
               Some(Claim{id: 1, start_col: 2, start_row: 3, num_cols: 4, num_rows: 5}));
    assert_eq!(parse_claim("#123 @ 596,731: 11x27"),
               Some(Claim{id: 123, start_col: 596, start_row: 731, num_cols: 11, num_rows: 27}));
    assert_eq!(parse_claim("Harrowdown Hill"), None);
}

// Dumb simulation-based solution.
fn solve_part1_naive(claims: &Vec<Claim>) -> i32 {
    // Paint the claims on the canvas.
    let canvas_num_rows = claims.iter().map(|c| c.start_row + c.num_rows).max().unwrap() as usize + 1;
    let canvas_num_cols = claims.iter().map(|c| c.start_col + c.num_cols).max().unwrap() as usize + 1;
    let mut canvas: Vec<i32> = Vec::new();  // row-major indexing
    canvas.resize(canvas_num_rows * canvas_num_cols, 0);
    for claim in claims {
        for r in 0..claim.num_rows {
            for c in 0..claim.num_cols {
                canvas[(claim.start_row + r) * canvas_num_cols + (claim.start_col + c)] += 1;
            }
        }
    }

    // // Print the canvas for debugging.
    // for r in 0..canvas_num_rows {
    //     for c in 0..canvas_num_cols {
    //         print!("{}", canvas[r * canvas_num_cols + c]);
    //     }
    //     print!("\n");
    // }

    // Count the number of cells that are part of multiple claims.
    let mut num_conflicts = 0;
    for r in 0..canvas_num_rows {
        for c in 0..canvas_num_cols {
            if canvas[r * canvas_num_cols + c] > 1 {
                num_conflicts += 1;
            }
        }
    }
    num_conflicts
}

#[test]
fn test_part1() {
    let claims = vec![
        parse_claim("#1 @ 1,3: 4x4").unwrap(),
        parse_claim("#2 @ 3,1: 4x4").unwrap(),
        parse_claim("#3 @ 5,5: 2x2").unwrap()];
    assert_eq!(solve_part1_naive(&claims), 4);
}

fn main() {
    let mut claims: Vec<Claim> = Vec::new();
    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line)
            .expect("Failed to read line");
        if line.is_empty() {
            break;
        }
        claims.push(parse_claim(line.trim()).unwrap());
    }
    println!("part 1: {:?}", solve_part1_naive(&claims));
    // println!("part 2: {:?}", solve_part2(&ids));
}
