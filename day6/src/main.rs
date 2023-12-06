use std::fs;

fn parse_line(line: &str) -> Vec<u64> {
    line.split_whitespace().skip(1).
        map(|x| x.trim().parse::<u64>().unwrap()).collect()
}

// For Part 2, parse the line as a concatenation of digits
fn parse_line2(line: &str) -> u64 {
    let nums : Vec<&str> = line.split(":").collect();
    let ret_str = nums[1].replace(" ", "");

    return ret_str.parse::<u64>().unwrap();
}

// given the distance we're trying to beat, the time given, and
// the time we hold down the button, will this beat the record?
fn is_winner(time: u64, dist: u64, hold_time: u64) -> bool {
    let speed = hold_time;
    let run_time = time - hold_time;
    let run_dist = run_time * speed;

    if run_dist > dist {
        return true;
    }

    return false;
}

fn main() {

    let binding = fs::read_to_string("input.txt").
        expect("Something went wrong reading the file");
    let mut lines = binding.lines();
    
    let times = parse_line(lines.next().unwrap());
    let dists = parse_line(lines.next().unwrap());

    let mut possible_wins: i32 = 1;

    // brute-force it.
    for race in 0..times.len() {
        let mut ways_to_win = 0;

        for hold_time in 0..times[race] {
            if is_winner(times[race], dists[race], hold_time) {
                ways_to_win += 1;
            }
        }
        possible_wins *= ways_to_win;
    }

    println!("Part 1 answer: {}", possible_wins);

    lines = binding.lines();
    let time = parse_line2(lines.next().unwrap());
    let dist = parse_line2(lines.next().unwrap());

    // We expect a min winning hold time and a max hold time, and everything in
    // between is a winner. We'll do a binary search of hold times, looking
    // for hold time that produces a loser next to a hold time that produces
    // a winner. The value of the winner is the min hold time. We'll do the
    // same but looking for a winner next to a loser, and the value of the
    // winner there is the max hold time. Max hold time - min hold time is
    // the number of winners.
    //
    // LLLLLLLLLLWWWWWWWWWWWWWLLLLLLLLLLLLL
    //          ^^           ^^
    //          min          max
    
    let mut win_low : u64 = 0;      // min successful hold time
    let mut win_high : u64 = 0;     // max successful hold time

    // search for min successful hold time in lower half to start.
    let mut low : u64 = 0;
    let mut high : u64 = time / 2;

    loop {
        let try_hold = low + (high - low) / 2;

        let left_win = is_winner(time, dist, try_hold);
        let right_win = is_winner(time, dist, try_hold + 1);

        if left_win && right_win {
            // two winners next to each other, ignore everything to our right
            high = try_hold - 1;
        } else if !left_win && !right_win {
            // two losers next to each other, ignore everything to our left
            low = try_hold + 1;
        } else if !left_win && right_win {
            // we found a loser, winner, we found lower bounds.
            win_low = try_hold;
            break;
        } else if left_win && !right_win {
            // we found a winner, loser, we found upper bounds.
            win_high = try_hold;
            break;
        }
    }

    // search again, but this time we expect winners to our left, losers to our right
    low = time / 2;
    high = time;

    loop {
        let try_hold = low + (high - low) / 2;

        let left_win = is_winner(time, dist, try_hold);
        let right_win = is_winner(time, dist, try_hold + 1);

        if left_win && right_win {
            low = try_hold + 1;
        } else if !left_win && !right_win {
            high = try_hold - 1;
        } else if !left_win && right_win {
            win_low = try_hold;
            break;
        } else if left_win && !right_win {
            win_high = try_hold;
            break;
        }
    }

    println!("Part 2 answer: {}", win_high - win_low);
}
