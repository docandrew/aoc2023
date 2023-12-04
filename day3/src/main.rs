// use std::cmp::{min, max};
use std::fs;
use std::collections::HashMap;
use array2d::Array2D;

fn main() {
    println!("############################");
    println!("          Part 1");
    println!("############################");
    part1();

    println!();
    println!("############################");
    println!("          Part 2");
    println!("############################");
    part2();
}

fn part1() {
    let contents = fs::read_to_string("input.txt").
        expect("Something went wrong reading the file");
 
    let mut matrix = Array2D::filled_with('A', 140, 140);
    let mut sum = 0;
    let mut row: usize = 0;
    let mut col: usize = 0;

    // load 2-d matrix
    for c in contents.chars() {
        if c == '\n' {
            row += 1;
            col = 0;
        } else {
            matrix[(row, col)] = c;
            col += 1;
        }
    }

    // Iterate over matrix until we find a digit. 
    // Keep chomping to find the full number, and identify the boundary of coords "around"
    // the number to look for a symbol. If we find a symbol, then read number into u32 and
    // add to sum. Then skip to end of number.

    for row in 0..140 {
        col = 0;
        while col < 140 {
            if matrix[(row, col)].is_digit(10) {
                println!("Found digit at [{}, {}]", row, col);
                let start_idx = col;
                let mut end_idx = col;

                while end_idx < 140 {
                    if matrix[(row, end_idx)].is_digit(10) {
                        end_idx += 1;
                    } else {
                        break;
                    }
                }

                let num_str: String = matrix.as_rows()[row][start_idx..end_idx].into_iter().collect();
                let num: u32 = num_str.parse().unwrap();

                let startrow: usize = if row == 0 {
                    0
                } else {
                    row - 1
                };

                let endrow: usize = if row >= 138 {
                    140
                } else {
                    row + 2
                };

                let startcol: usize = if start_idx == 0 {
                    0
                } else {
                    start_idx - 1
                };

                let endcol: usize = if end_idx >= 138 {
                    140
                } else {
                    end_idx + 1
                };

                println!("Found num {} at [{}, {}], searching for surrounding symbol from [{}..{}, {}..{}]", num, row, col, startrow, endrow, startcol, endcol);

                'outer: for searchrow in startrow .. endrow {
                    for searchcol in startcol .. endcol {
                        print!("{}",matrix[(searchrow,searchcol)]);
                        if matrix[(searchrow, searchcol)].is_ascii_punctuation() && matrix[(searchrow, searchcol)] != '.' {
                            sum += num;
                            println!();
                            println!("Found adjacent symbol {}, adding {} to sum, sum: {}", matrix[(searchrow, searchcol)], num, sum);
                            break 'outer;
                        }
                    }
                    println!();
                }

                col = end_idx;
            } else {
                col += 1;
            }
        }
        println!();
    }

    println!("Part 1 Answer: {}", sum);
}

fn part2() {

    let contents = fs::read_to_string("input.txt").
        expect("Something went wrong reading the file");
 
    let mut matrix = Array2D::filled_with('A', 140, 140);
    let mut row: usize = 0;
    let mut col: usize = 0;

    // load 2-d matrix
    for c in contents.chars() {
        if c == '\n' {
            row += 1;
            col = 0;
        } else {
            matrix[(row, col)] = c;
            col += 1;
        }
    }

    // For this part, we'll do the same thing we did in Part 1, but 
    // only search for the '*' gear symbol, and keep track of the
    // coordinate of this gear along with the adjacent number. 
    // We'll use a HashMap<(usize, usize), Vec<u32>>.
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for row in 0..140 {
        col = 0;
        while col < 140 { 
            if matrix[(row, col)].is_digit(10) {
                println!("Found digit at [{}, {}]", row, col);
                let start_idx = col;
                let mut end_idx = col;

                while end_idx < 140 {
                    if matrix[(row, end_idx)].is_digit(10) {
                        end_idx += 1;
                    } else {
                        break;
                    }
                }

                let num_str: String = matrix.as_rows()[row][start_idx..end_idx].into_iter().collect();
                let num: u32 = num_str.parse().unwrap();

                let startrow: usize = if row == 0 {
                    0
                } else {
                    row - 1
                };

                let endrow: usize = if row >= 138 {
                    140
                } else {
                    row + 2
                };

                let startcol: usize = if start_idx == 0 {
                    0
                } else {
                    start_idx - 1
                };

                let endcol: usize = if end_idx >= 138 {
                    140
                } else {
                    end_idx + 1
                };

                println!("Found num {} at [{}, {}], searching for surrounding * from [{}..{}, {}..{}]", num, row, col, startrow, endrow, startcol, endcol);

                // Each time we find a number adjacent to the gear, go ahead and append that number to
                // the list of numbers for that gear.
                'outer: for searchrow in startrow .. endrow {
                    for searchcol in startcol .. endcol {
                        print!("{}",matrix[(searchrow,searchcol)]);
                        if matrix[(searchrow, searchcol)] == '*' {
                            println!();
                            if !gears.contains_key(&(searchrow, searchcol)) {
                                println!("Adding gear at coordinate [{}, {}]", searchrow, searchcol);
                                gears.insert((searchrow, searchcol), Vec::new());
                            }

                            println!(" Adding {} to list of numbers adjacent to gear [{}, {}]", num, searchrow, searchcol);
                            gears.get_mut(&(searchrow, searchcol)).unwrap().push(num);
                            
                            break 'outer;
                        }
                    }
                    println!();
                }

                col = end_idx;
            } else {
                col += 1;
            }
        }
        println!();
    }

    let mut sum = 0;

    // Now with all gears adjacent to at least one number, iterate through them.
    // If they are adjacent to exactly 2 numbers, go ahead and multiply them and add to sum.
    for (gear, adjacent_nums) in &gears {
        if adjacent_nums.len() == 2 {
            let product = adjacent_nums[0] * adjacent_nums[1];
            sum += product;
            println!("Gear at [{}, {}] has adjacent nums {}, {}, adding their product {} to sum: {}", gear.0, gear.1, adjacent_nums[0], adjacent_nums[1], product, sum);
        } else {
            println!("Gear found with {} adjacent nums, skipping.", adjacent_nums.len());
        }
    }

    println!("Part 2 answer: {}", sum);
}
