use std::collections::HashSet;
use std::io;

fn reduces(a: char, b: char) -> bool {
    (a.is_lowercase() ^ b.is_lowercase()) &&
    (a.to_lowercase().to_string() == b.to_lowercase().to_string())
}

fn solve_part1<T: AsRef<str>>(input: &T) -> String {
    let mut stack: Vec<char> = Vec::new();
    for c in input.as_ref().chars() {
        if !stack.is_empty() && reduces(*stack.last().unwrap(), c) {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    stack.iter().collect()
}

#[test]
fn test_solve_part1() {
    assert_eq!(solve_part1(&"aA"), "".to_string());
    assert_eq!(solve_part1(&"abBA"), "".to_string());
    assert_eq!(solve_part1(&"abAB"), "abAB".to_string());
    assert_eq!(solve_part1(&"aabAAB"), "aabAAB".to_string());
    assert_eq!(solve_part1(&"dabAcCaCBAcCcaDA"), "dabCBAcaDA".to_string());
}

// Returns (lowercase_char_to_remove, resulting_reduced_str).
// Naive brute-force implementation.
fn solve_part2<T: AsRef<str>>(input: &T) -> (char, String) {
    let mut lowercase_chars: HashSet<char> = HashSet::new();
    for c in input.as_ref().chars() {
        let c_lower = c.to_lowercase().to_string();
        assert!(c_lower.len() == 1);
        lowercase_chars.insert(c_lower.chars().next().unwrap());
    }
    println!("Trying each of {} chars to remove...", lowercase_chars.len());
    let mut best_char_to_remove = '?';
    let mut best_reduced_str = String::new();
    let mut best_reduced_str_len = input.as_ref().len();
    for char_to_remove in lowercase_chars.iter() {
        let mut stack: Vec<char> = Vec::new();
        for c in input.as_ref().chars() {
            let c_lower = c.to_lowercase().to_string();
            assert!(c_lower.len() == 1);
            if c_lower.chars().next().unwrap() == *char_to_remove {
                continue;
            }
            if !stack.is_empty() && reduces(*stack.last().unwrap(), c) {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        let reduced_str: String = stack.iter().collect();
        let reduced_str_len = reduced_str.len();
        if reduced_str_len < best_reduced_str_len {
            best_char_to_remove = *char_to_remove;
            best_reduced_str = reduced_str;
            best_reduced_str_len = reduced_str_len;
        }
    }
    assert!(best_char_to_remove != '?');
    (best_char_to_remove, best_reduced_str)
}

#[test]
fn test_solve_part2() {
    assert_eq!(solve_part2(&"dabAcCaCBAcCcaDA"), ('c', "daDA".to_string()));
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    println!("part 1: {:?}", solve_part1(&input.trim()).len());
    let (char_to_remove, reduced_str) = solve_part2(&input.trim());
    println!("part 2: {:?} {:?}", char_to_remove, reduced_str.len());
}

