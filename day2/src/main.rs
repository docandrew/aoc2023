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
    
    // max number of cubes we can draw per "grab"
    let max_red: i32 = 12;
    let max_green: i32 = 13;
    let max_blue: i32 = 14;

    // mutable var for running sum
    let mut sum = 0;

    for line in contents {
        
        // game is valid until proven otherwise
        let mut valid_game: bool = true;

        // parse game ID, will be column 5 until colon
        let colon_idx: usize = line.find(':').unwrap();
        let game_num_str: String = line[5..colon_idx].to_string();
        let game_num: i32 = game_num_str.parse().unwrap();
        println!("Game number {} ", game_num);

        // check each "draw" of cubes
        let parts: Vec<&str> = line[colon_idx+1..].split_terminator(&[':', ';'][..]).collect();

        for part in parts {
            // split by comma
            let color_draw: Vec<&str> = part.split(',').map(|l| l.trim()).collect();

            // now two parts, a number and a color
            for color in color_draw {
                
                let space_idx: usize = color.find(' ').unwrap();
                let color_num_str: String = color[..space_idx].to_string();
                let color_num: i32 = color_num_str.parse().unwrap();
                print!("{} ", color_num);

                let color_val_str: String = color[space_idx+1..].to_string();
                print!("{} ", color_val_str);
                match color_val_str.as_str() {
                    "red" => 
                        if color_num > max_red { 
                            valid_game = false;
                            print!(" Too many red: {} ", color_num);
                            break; 
                        },
                        "green" => 
                        if color_num > max_green { 
                            valid_game = false;
                            print!(" Too many green: {} ", color_num);
                            break;
                        },
                        "blue" =>
                        if color_num > max_blue {
                            valid_game = false;
                            print!(" Too many blue: {} ", color_num);
                            break;
                        },
                    _ => println!("BAD COLOR VALUE, DID YOU PARSE SOMETHING WRONG?")
                }
            }

            if !valid_game {
                break;
            }
        }

        if valid_game {
            sum += game_num;
            println!("Game {} is valid, sum: {}", game_num, sum);
        } else {
            println!("Game {} is invalid.", game_num);
        }
    }

    println!("Sum of valid game IDs: {}", sum);
}

fn part2() {
    let binding = fs::read_to_string("input.txt").
        expect("Something went wrong reading the file");
    let contents = binding.split("\n");
    
    let mut sum = 0;
    
    for line in contents {
        // max number of cubes we've seen per "grab"
        let mut max_red: i32 = 0;
        let mut max_green: i32 = 0;
        let mut max_blue: i32 = 0;
        
        let colon_idx: usize = line.find(':').unwrap();
        let game_num_str: String = line[5..colon_idx].to_string();
        let game_num: i32 = game_num_str.parse().unwrap();

        let parts: Vec<&str> = line[colon_idx+1..].split_terminator(&[':', ';'][..]).collect();

        for part in parts {
            let color_draw: Vec<&str> = part.split(',').map(|l| l.trim()).collect();

            for color in color_draw {
                
                let space_idx: usize = color.find(' ').unwrap();
                let color_num_str: String = color[..space_idx].to_string();
                let color_num: i32 = color_num_str.parse().unwrap();

                let color_val_str: String = color[space_idx+1..].to_string();
                match color_val_str.as_str() {
                    "red" => 
                        if color_num > max_red { 
                            max_red = color_num;
                        },
                    "green" => 
                        if color_num > max_green { 
                            max_green = color_num;
                        },
                    "blue" =>
                        if color_num > max_blue {
                            max_blue = color_num;
                        },
                    _ => println!("BAD COLOR VALUE, DID YOU PARSE SOMETHING WRONG?")
                }
            }
        }

        let game_power = max_red * max_green * max_blue;

        sum += game_power;

        println!("Game {}, min red/green/blue: {} {} {} power: {} sum: {}", game_num, max_red, max_green, max_blue, game_power, sum);
    }

    println!("Sum of game powers: {}", sum);
}
