use std::io;
use std::collections::HashMap;

fn solve_part1(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    sum
}

// Represents a solution. Interpretation:
//      value == sum(numbers[:i-1])
//            == sum(numbers[:j-1]) + k * sum(numbers).
#[derive(Debug)]
struct Solution {
    value: i32,
    i: i32,
    j: i32,
    k: i32,
}

impl Solution {
    // Returns true iff self is an earlier (and therefore better) solution than other.
    fn better_than(&self, other: &Solution) -> bool {
        (self.k, self.j, self.i) < (other.k, other.j, other.i)
    }
}

// Returns the solution, or None if there is no solution.
fn solve_part2_naive(numbers: &Vec<i32>) -> Option<Solution> {
    // First pass. If a solution is found, return it immediately.
    let mut sum_to_i: HashMap<i32, i32> = HashMap::new();
    sum_to_i.insert(0, -1);
    let mut sum = 0;
    for (j, number) in numbers.iter().enumerate() {
        sum += number;
        match sum_to_i.get(&sum) {
            Some(i) => return Some(Solution{value: sum, i: *i, j: j as i32, k: 0}),
            None => ()
        };
        sum_to_i.insert(sum, j as i32);
    }

    // Second pass. Look for solutions that involve going through the input sequence multiple
    // times. If there is a solution, then it must involve one of the values seen in the first
    // pass. Consider: sum(numbers[:i-1]) + ki * sum(numbers) == sum(numbers[:j-1]) + kj *
    // sum(numbers). Then we can find a solution with ki = 0 by subtracting a multiple of
    // sum(numbers) from both sides. Therefore we do not need to add more entries to sum_to_i for k
    // > 0. When all the values we see have gone outside the range for k = 0, then no solution is
    // possible.
    let range_min = sum_to_i.keys().min().unwrap();
    let range_max = sum_to_i.keys().max().unwrap();
    let mut k = 1;
    loop {
        let mut found_value_in_range = false;
        for (j, number) in numbers.iter().enumerate() {
            sum += number;
            match sum_to_i.get(&sum) {
                Some(i) => return Some(Solution{value: sum, i: *i, j: j as i32, k}),
                None => ()
            };
            if sum >= *range_min && sum <= *range_max {
                found_value_in_range = true;
            }
        }
        if !found_value_in_range {
            return None
        }
        k += 1;
    }
}

// Info about an index in the input numbers array. Interpretation:
// numbers[index] = multiple * sum(numbers) + modulo.
#[derive(Copy)]
#[derive(Debug)]
struct IndexInfo {
    index: i32,
    multiple: i32,
    modulo: i32,
}

impl Clone for IndexInfo {
    fn clone(&self) -> IndexInfo { *self }
}

// As above, but using a fancy O(n log n) solution.
fn solve_part2_fast(numbers: &Vec<i32>) -> Option<Solution> {
    let total: i32 = numbers.iter().sum();
    // println!("total: {}", total);

    // Case 1: We have k = 0. Get the solution in O(n).
    if total == 0 {
        let mut sum_to_i: HashMap<i32, i32> = HashMap::new();
        sum_to_i.insert(0, -1);
        let mut sum = 0;
        for (j, number) in numbers.iter().enumerate() {
            sum += number;
            match sum_to_i.get(&sum) {
                Some(i) => return Some(Solution{value: sum, i: *i, j: j as i32, k: 0}),
                None => ()
            };
            sum_to_i.insert(sum, j as i32);
        }
        panic!("should not reach here");
    }

    // Case 2: We have k > 0. Get the solution in O(n log n).
    // Group the sum values by their modulo when divided by total.
    let mut sums: Vec<i32> = Vec::new();
    let mut sum = 0;
    let mut mod_to_index_infos: HashMap<i32, Vec<IndexInfo>> = HashMap::new();
    let empty_index_infos: Vec<IndexInfo> = Vec::new();
    for (j, number) in numbers.iter().enumerate() {
        sum += number;
        sums.push(sum);
        // Major gotcha: In Python % is modulus, but in C and Rust, % is remainder... Also in
        // Python 2.7 / is floor division, but in C and Rust / is division with truncation towards
        // zero... So these work completely differently for negative numbers.
        let modulo = (sum % total + total) % total;
        let multiple = (sum as f64 / total as f64).floor() as i32;
        assert!(modulo >= 0 && modulo < total);
        assert!(multiple * total + modulo == sum);
        if !mod_to_index_infos.contains_key(&modulo) {
            mod_to_index_infos.insert(modulo, empty_index_infos.clone());
        }
        let index_infos = mod_to_index_infos.get_mut(&modulo).unwrap();
        index_infos.push(IndexInfo{index: j as i32, multiple: sum / total, modulo: modulo});
    }
    // If i and j form a solution, they must be in the same modulo group.
    let mut best_soln: Option<Solution> = None;
    for (_modulo, index_infos) in mod_to_index_infos.iter_mut() {
        index_infos.sort_unstable_by(|a, b| a.multiple.cmp(&b.multiple));
        // println!("modulo={} index_infos={:?}", _modulo, index_infos);
        for t in 1..index_infos.len() {
            // println!("t={}", t);
            let i = index_infos[t];
            let j = index_infos[t - 1];
            assert!(i.multiple >= j.multiple);
            let k = i.multiple - j.multiple;
            let soln = Solution{value: sums[i.index as usize], i: i.index, j: j.index, k};
            let soln_is_better = match best_soln {
                None => true,
                Some(ref other_soln) => soln.better_than(&other_soln)
            };
            // println!("candidate: {:?} better={}", soln, soln_is_better);
            if soln_is_better {
                best_soln = Some(soln);
            }
        }
    }
    best_soln
}

fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line)
            .expect("Failed to read line");
        if line.is_empty() {
            break;
        }
        let line_int: i32 = line.trim().parse()
            .expect("Could not convert to i32");
        numbers.push(line_int);
    }
    println!("part 1: {}", solve_part1(&numbers));
    println!("part 2 naive: {:?}", solve_part2_naive(&numbers));
    println!("part 2 fast: {:?}", solve_part2_fast(&numbers));
}
