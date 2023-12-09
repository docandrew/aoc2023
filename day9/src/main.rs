use std::fs;
use std::collections::VecDeque;

// given a vector of nums, predict the next value of the history
// recursively.
fn predict_next(v : &Vec<i32>) -> i32 {
    
    // check for all zeroes, this is the base case.
    let mut all_zeroes = true;

    for i in v {
        if *i != 0 {
            all_zeroes = false;
            break;
        }
    }

    if all_zeroes {
        return 0;
    }

    let mut next_vec : Vec<i32> = Vec::new();

    // determine next vector by subtracting each element from the last
    for i in 0..v.len()-1 {
        next_vec.push(v[i+1] - v[i]);
    }

    return v.last().unwrap() + predict_next(&next_vec);
}

fn predict_prev(v : &VecDeque<i32>) -> i32 {
    
    // check for all zeroes, this is the base case.
    let mut all_zeroes = true;

    for i in v {
        if *i != 0 {
            all_zeroes = false;
            break;
        }
    }

    if all_zeroes {
        return 0;
    }

    let mut next_vec : VecDeque<i32> = VecDeque::new();

    // determine next vector by subtracting each element from the last as in part 1
    for i in 0..v.len()-1 {
        next_vec.push_back(v[i+1] - v[i]);
    }

    return v.front().unwrap() - predict_prev(&next_vec);
}

fn part1(filename : &str) {
    let binding = fs::read_to_string(filename).
        expect("Something went wrong reading the file");

    let mut sum = 0;

    for history_str in binding.lines() {
        let history : Vec<i32> = history_str.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        let next = predict_next(&history);
        sum += next;
    }

    println!("Part 1 answer: {}", sum);
}

fn part2(filename : &str) {
    let binding = fs::read_to_string(filename).
        expect("Something went wrong reading the file");

    let mut sum = 0;

    for history_str in binding.lines() {
        let history : VecDeque<i32> = history_str.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        let prev = predict_prev(&history);
        sum += prev;
    }

    println!("Part 2 answer: {}", sum);
}

fn main() {
    part1("input.txt");
    part2("input.txt");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        super::part1("input_test.txt");
    }

    #[test]
    fn test_part2() {
        super::part2("input_test.txt");
    }
}
