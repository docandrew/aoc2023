use std::fs;
use std::collections::HashMap;
use num::integer::lcm;

struct Node {
    //label : String,
    left  : String,
    right : String,
}

fn steps_to_zzz (starting_node : String, s : String) -> usize {
    let contents : Vec<&str> = s.split("\n").collect();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let path = contents[0].to_string();

    // Grr - instructions say "start with AAA", NOT the first line. :facepalm:
    let mut cur : String = starting_node;

    // parse nodes
    for line in contents[2..].iter() {
        let parsable_line = line.replace(" = (", " ")
                .replace(", ", " ")
                .replace(")","");
        
        let elements = parsable_line.split(" ").collect::<Vec<&str>>();

        let label = elements[0];
        let left  = elements[1];
        let right = elements[2];

        nodes.insert(label.to_string(), Node{left: left.to_string(), 
                                                  right: right.to_string()});
    }

    let mut steps = 0;
    let mut path_idx = 0;

    while cur != "ZZZ" {
        if path_idx == path.len() {
            path_idx = 0;
        }

        let c : char = path.as_bytes()[path_idx] as char;
        match c {
            'L' => {
                cur = nodes.get(&cur).unwrap().left.to_string();
            },
            'R' => {
                cur = nodes.get(&cur).unwrap().right.to_string();
            },
            _ => {
                println!("Invalid character in path: {}", c);
            }
        }

        path_idx += 1;
        steps += 1;
    }

    return steps;
}

#[derive(PartialEq, Eq, Debug)]
struct SeenNode {
    label : String,
    path_idx : usize,
    steps : usize,
}

// Part 2. Find paths to xxZ nodes. We'll keep going until we hit the same
// node at the same place in the path, then we'll know we've found a cycle of Z
// nodes. We'll return a vector of the visited Z nodes with their respective
// number of steps. The least common multiple of these will be the answer.
fn steps_to_xxz (starting_node : String, s : String) -> usize {
    let contents : Vec<&str> = s.split("\n").collect();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let path = contents[0].to_string();
    let mut cur : String = starting_node;

    for line in contents[2..].iter() {
        let parsable_line = line.replace(" = (", " ")
                .replace(", ", " ")
                .replace(")","");
        
        let elements = parsable_line.split(" ").collect::<Vec<&str>>();

        let label = elements[0];
        let left  = elements[1];
        let right = elements[2];

        nodes.insert(label.to_string(), Node{left: left.to_string(), 
                                                  right: right.to_string()});
    }

    let mut steps = 1;
    let mut path_idx = 0;

    loop {
        if path_idx == path.len() {
            path_idx = 0;
        }

        let c : char = path.as_bytes()[path_idx] as char;
        match c {
            'L' => {
                cur = nodes.get(&cur).unwrap().left.to_string();
            },
            'R' => {
                cur = nodes.get(&cur).unwrap().right.to_string();
            },
            _ => {
                println!("Invalid character in path: {}", c);
            }
        }

        // if we see the same node at the same place in the path, we've found
        // a cycle, and can stop, since the first time we see it will always
        // be the same cycle with this input data.
        if cur.as_bytes()[2] as char == 'Z' {
            return steps;
        }

        path_idx += 1;
        steps += 1;
    }
}

fn main() {
    let binding = fs::read_to_string("input.txt").
        expect("Something went wrong reading the file");

    println!("Part 1 answer: {}", steps_to_zzz("AAA".to_string(), binding.clone()));
    
    // for each of the starting nodes in part 2, find the path to a xxZ node,
    // record the number of steps. The least common multiple of all of these
    // will be the answer.
    let lpa = steps_to_xxz("LPA".to_string(), binding.clone());
    let aaa = steps_to_xxz("AAA".to_string(), binding.clone());
    let qga = steps_to_xxz("QGA".to_string(), binding.clone());
    let hha = steps_to_xxz("HHA".to_string(), binding.clone());
    let xka = steps_to_xxz("XKA".to_string(), binding.clone());
    let lta = steps_to_xxz("LTA".to_string(), binding.clone());

    let lcm = lcm(lpa, lcm(aaa, lcm(qga, lcm(hha, lcm(xka, lta)))));
    println!("Part 2 answer: {}", lcm);
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_case() {
        let input1 = fs::read_to_string("input_test.txt").
            expect("Something went wrong reading the file");
        let input2 = fs::read_to_string("input_test2.txt").
            expect("Something went wrong reading the file");

        assert_eq!(super::steps_to_zzz("AAA".to_string(), input1), 2);
        assert_eq!(super::steps_to_zzz("AAA".to_string(), input2), 6);
    }
}
