use std::fs;

fn main() {
    println!("############################");
    println!("          Part 1");
    println!("############################");
    part1();

    println!("############################");
    println!("          Part 2");
    println!("############################");
    part2();
}

fn part1() {
    let binding = fs::read_to_string("input.txt").
        expect("Something went wrong reading the file");
    let contents = binding.split("\n");
    
    // mutable var for running sum
    let mut sum = 0;

    for line in contents {
        let mut first_number = 0;
        let mut second_number = 0;

        // search from beginning to find first number
        for i in line.chars() {
            // is a number?
            if i.is_digit(10) {
                // convert char to integer
                first_number = i.to_digit(10).unwrap();
                break;
            }
        }
        // search from end to find last number
        for i in line.chars().rev() {
            if i.is_digit(10) {
                second_number = i.to_digit(10).unwrap();
                break;
            }
        }

        sum += (first_number * 10) + second_number;
        println!("{} {} {} {}", line, first_number, second_number, sum);
    }
}

fn part2() {
    // in each line, we'll do string substitution to replace number "words"
    // with their respective digit, then re-run the same logic as part 1.
    // this can cause problems if the number words are part of other words,
    // i.e. eightwo. In this case we'll want "eight", since it comes first,
    // unless its at the end, then we'll want "two". To get around this we'll
    // preserve the first and last letters of each number word after substitution.
    // So oneight becomes "o1eight" after substituting "one" and then "o1e8t"
    // after substituting "eight". So this gives the correct result of 18.
    let binding = fs::read_to_string("input.txt").
        expect("Something went wrong reading the file").
        replace("one", "o1e").
        replace("two", "t2o").
        replace("three", "t3e").
        replace("four", "f4r").
        replace("five", "f5e").
        replace("six", "s6x").
        replace("seven", "s7n").
        replace("eight", "e8t").
        replace("nine", "n9e");

    let contents = binding.split("\n");

    let mut sum = 0;

    for line in contents {
        let mut first_number = 0;
        let mut second_number = 0;

        for i in line.chars() {
            if i.is_digit(10) {
                first_number = i.to_digit(10).unwrap();
                break;
            }
        }

        for i in line.chars().rev() {
            if i.is_digit(10) {
                second_number = i.to_digit(10).unwrap();
                break;
            }
        }

        sum += (first_number * 10) + second_number;

        println!("{} {} {} {}", line, first_number, second_number, sum);
    }
}
