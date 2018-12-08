use std::io;
use std::collections::HashSet;

fn print_grid(grid: &Vec<(i32, i32)>, num_rows: i32, num_cols: i32) {
    println!("=========================");
    for r in 0..num_rows {
        for c in 0..num_cols {
            let (i, d) = grid[(r * num_cols + c) as usize];
            print!("(i={:3} d={:3})   ", i, d);
        }
        println!();
    }
    println!("=========================");
}

// Returns the size of the largest finite Voronoi cell (using Manhattan distance).
// This does not actually compute the Voronoi diagram directly; instead it uses BFS
// to color the grid starting from the input coords, then their d=1 neighbors, etc.
// Notice that we don't need to look outside the rectangle defined by min/max input
// coordinate along x and y. If a grid cell c along the edges of that rectangle is
// colored by some index i, then any cell c' farther out in that direction will also
// be colored i, because d(c', j) == d(c, j) + 1 for all j.
//
// Ugh, this turned out to be pretty hairy. Should've used even bruter force.
fn solve_part1_naive(coords: &Vec<(i32, i32)>) -> i32 {
    // Figure out the size of the grid.
    let min_r = coords.iter().map(|p| p.0).min().unwrap();
    let max_r = coords.iter().map(|p| p.0).max().unwrap();
    let min_c = coords.iter().map(|p| p.1).min().unwrap();
    let max_c = coords.iter().map(|p| p.1).max().unwrap();
    let num_rows = max_r - min_r + 1;
    let num_cols = max_c - min_c + 1;
    
    // Determines whether a given cell is inside the grid.
    let is_inside_grid = |r: i32, c: i32|
            r >= min_r && r <= max_r && c >= min_c && c <= max_c;

    // Computes the index in the grid for a given row and column.
    let get_grid_index = |r: i32, c: i32|
            ((r - min_r) * num_cols + (c - min_c)) as usize;

    // For each cell, holds (closest_coord_index, closest_coord_dist).
    let mut grid: Vec<(i32, i32)> = Vec::new();
    const NOT_VISITED: i32 = -1;
    const EQUIDISTANT: i32 = -2;
    grid.resize((num_rows * num_cols) as usize, (NOT_VISITED, -1));

    // Counts the number of cells that are closest to each input coordinate.
    let mut index_to_area: Vec<i32> = Vec::new();
    index_to_area.resize(coords.len(), 0);

    // Holds the current distance, and the cells that would get that distance
    // in the next iteration.
    let mut current_dist = 0;
    let mut frontier: Vec<(i32, i32, i32)> = Vec::new();  // (coord_index, r, c) tuples
    for (i, (r, c)) in coords.iter().enumerate() {
        frontier.push((i as i32, *r, *c));
    }

    // BFS.
    while !frontier.is_empty() {
        // println!("Iter for current_dist={}", current_dist);

        let mut newly_visited_cells: Vec<(i32, i32)> = Vec::new();
        for (i_ref, r_ref, c_ref) in frontier.iter() {
            let i = *i_ref;
            let r = *r_ref;
            let c = *c_ref;
            // print!("  i={} r={} c={} rr={} cc={}", i, r, c, r - min_r, c - min_c);
            if !is_inside_grid(r, c) {
                // println!(" outside grid");
                continue;
            }
            let (existing_index, existing_dist) = grid[get_grid_index(r, c)];
            if existing_index == NOT_VISITED {
                // This cell was never visited before. Count it as closest to i.
                // println!(" first visit");
                grid[get_grid_index(r, c)] = (i, current_dist);
                if i != EQUIDISTANT {
                    index_to_area[i as usize] += 1;
                }
                newly_visited_cells.push((r, c));
            } else if existing_dist == current_dist {
                if existing_index != i {
                    // This cell was visited before in this iteration, and assigned to
                    // a different coord. This means it is the same distance away from
                    // multiple coords.
                    // println!(" equidistant");
                    if existing_index != EQUIDISTANT {
                        index_to_area[existing_index as usize] -= 1;
                    }
                    grid[get_grid_index(r, c)] = (EQUIDISTANT, current_dist);
                }
            } else {
                // This cell was visited in a previous iteration.
                // println!(" previously visited");
                assert!(existing_dist < current_dist);
            }
        }

        frontier.clear();
        for (r, c) in newly_visited_cells {
            let (index, dist) = grid[get_grid_index(r, c)];
            // println!("r={} c={} i={} d={}", r, c, index, dist);
            assert!(index != NOT_VISITED);
            assert!(dist == current_dist);
            frontier.push((index, r, c + 1));
            frontier.push((index, r, c - 1));
            frontier.push((index, r + 1, c));
            frontier.push((index, r - 1, c));
        }
        current_dist += 1;

        // print_grid(&grid, num_rows, num_cols);
    }

    // Exclude coordinates whose Voronoi cells are infinite.
    let mut indices_with_infinite_cells: HashSet<i32> = HashSet::new();
    for r in min_r..(max_r - 1) {
        indices_with_infinite_cells.insert(grid[get_grid_index(r, min_c)].0);
        indices_with_infinite_cells.insert(grid[get_grid_index(r, max_c)].0);
    }
    for c in min_c..(max_c - 1) {
        indices_with_infinite_cells.insert(grid[get_grid_index(min_r, c)].0);
        indices_with_infinite_cells.insert(grid[get_grid_index(max_r, c)].0);
    }
    // println!("indices_with_infinite_cells: {:?}", indices_with_infinite_cells);

    let mut max_area: i32 = -1;
    for (index, area) in index_to_area.iter().enumerate() {
        let is_infinite = indices_with_infinite_cells.contains(&(index as i32));
        println!("index_to_area[{}] = {} (infinite={})", index, area, is_infinite);
        if !is_infinite && *area > max_area {
            max_area = *area;
        }
    }
    max_area
}

#[test]
fn test_solve_part1_naive() {
    let coords: Vec<(i32, i32)> = [(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)].to_vec();
    assert_eq!(solve_part1_naive(&coords), 17);
}

fn main() {
    let mut coords: Vec<(i32, i32)> = Vec::new();
    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line)
            .expect("Failed to read line");
        if line.is_empty() {
            break;
        }
        let tmp: Vec<&str> = line.trim().split(", ").collect();
        assert_eq!(tmp.len(), 2);
        coords.push((tmp[0].parse().unwrap(), tmp[1].parse().unwrap()));
    }
    println!("part 1: {:?}", solve_part1_naive(&coords));
}
