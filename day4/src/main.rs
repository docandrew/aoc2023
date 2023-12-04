use std::fs;
use std::collections::HashSet;

fn main() {
    part1_2();
}

fn part1_2() {
    println!("############################");
    println!("          Part 1");
    println!("############################");
    
    let contents = fs::read_to_string("input.txt").
    expect("Something went wrong reading the file");
    
    // save matches vector for part 2. Array would be faster but I don't want
    // to deal with indexing in the for_each loop below.
    let mut matches = Vec::new();

    let mut sum = 0;

    // Read in input.txt, parse each line into two sets of integers
    contents.lines().for_each(|line| {
        let parts = line.split([':', '|']).collect::<Vec<&str>>();
        let (_, winning_str, yournums_str) = (parts[0], parts[1], parts[2]);
        // println!("winning: {}, yours: {}", winning_str, yournums_str);
        let winning_nums = winning_str.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<HashSet<u32>>();
        let your_nums = yournums_str.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<HashSet<u32>>();

        let intersection: HashSet<_> = your_nums.intersection(&winning_nums).collect();
        // println!("{} nums in common: {:?}", intersection.len(), intersection);

        let mut points = 0;
        if intersection.len() > 0 {
            points = 2_u32.pow(intersection.len() as u32 - 1);
        }

        // println!("points: {}", points);
        matches.push(intersection.len());
        sum += points;
    });

    println!("Part 1 answer: {}", sum);

    println!("############################");
    println!("          Part 2");
    println!("############################");

    // part 2 is kind of a dynamic programming thing. The number of copies for
    // a given card is the sum of the number of copies for the next M cards,
    // where M is the number of matches for that given card.
    // so matches[M] is an vector from part 1.
    // copies[n] = copies[n+1] + copies[n+2] + ... + copies[n+M]
    // build up copies backwards from the last card to the first.
    let mut copies = vec![0; matches.len()];

    for i in (0..matches.len()).rev() {
        let mut sum_2 = 1;          // get 1 instance of the original card.
        for j in 0..matches[i] as usize {
            sum_2 += copies[i+j+1];
        }
        copies[i] = sum_2;
    }

    // sum all copies
    let sum: u32 = copies.iter().sum();

    println!("Part 2 answer: {}", sum);
}
