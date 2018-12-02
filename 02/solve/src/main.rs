// Comments lack context; see puzzle text for more info.

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

// Returns (num_2, num_3, num_2 * num_3).
// For the function type, see https://stackoverflow.com/a/41180422/744071.
fn solve_part1<T: AsRef<str>>(ids: &[T]) -> (i32, i32, i32) {
    let mut num_repeats_to_count : HashMap<i32, i32> = HashMap::new();
    for id in ids {
        let mut char_to_count : HashMap<char, i32> = HashMap::new();
        for c in id.as_ref().chars() {
            *(char_to_count.entry(c).or_insert(0)) += 1;
        }
        let mut num_repeats: Vec<i32> = char_to_count.values().map(|&c| c).collect();
        num_repeats.sort_unstable();
        num_repeats.dedup();
        for count in num_repeats.iter() {
            *(num_repeats_to_count.entry(*count).or_insert(0)) += 1;
        }
        // println!("id={} char_to_count={:?} num_repeats={:?}",
        //     id.as_ref(), char_to_count, num_repeats);
        // println!("num_repeats_to_count={:?}", num_repeats_to_count);
    }
    let num_2 = *(num_repeats_to_count.get(&2).unwrap_or(&0));
    let num_3 = *(num_repeats_to_count.get(&3).unwrap_or(&0));
    return (num_2, num_3, num_2 * num_3);
}

#[test]
fn test_part1() {
    assert_eq!(
        solve_part1(&vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"]),
        (4, 3, 12));
}

// Returns the common substring. O(num_strings * string_length^2).
// Did not spend any time looking for a more efficient solution.
fn solve_part2<T: AsRef<str>>(ids: &[T]) -> Option<String> {
    if ids.is_empty() {
        return None;
    }
    let id_len = ids[0].as_ref().len();
    for index_to_drop in 0..id_len {
        let mut seen: HashSet<String> = HashSet::new();
        for id_thingy in ids {
            let id = id_thingy.as_ref();
            assert!(id.len() == id_len);
            let mod_id = id[..index_to_drop].to_string() + &id[index_to_drop + 1..];
            if seen.contains(&mod_id) {
                return Some(mod_id);
            }
            seen.insert(mod_id);
        }
    }
    None
}

#[test]
fn test_part2() {
    assert_eq!(
        solve_part2(&vec!["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"]),
        Some("fgij".to_string()));
}

fn main() {
    let mut ids: Vec<String> = Vec::new();
    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line)
            .expect("Failed to read line");
        if line.is_empty() {
            break;
        }
        ids.push(line.trim().to_string());
    }
    println!("part 1: {:?}", solve_part1(&ids));
    println!("part 2: {:?}", solve_part2(&ids));
}
