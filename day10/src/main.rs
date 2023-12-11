use std::fs;
use array2d::Array2D;

// Load a maze from a file into a 2-d matrix. Also return the starting position
// that was marked with 'S'
fn load_maze(filename : &str, num_rows : usize, num_cols : usize, start_pos : &mut (usize, usize)) -> Array2D<char> {
    let contents = fs::read_to_string(filename).
    expect("Something went wrong reading the file");

    let mut maze = Array2D::filled_with('X', num_rows, num_cols);
    let mut row: usize = 0;
    let mut col: usize = 0;

    for c in contents.chars() {
        if c == '\n' {
            row += 1;
            col = 0;           
        } else {
            if c == 'S' {
                *start_pos = (row, col);
            }
            maze[(row, col)] = c;
            col += 1;
        }
    }

    return maze;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Given a maze, a starting position, and a starting direction, fill in the distances to all other reachable points
fn fill_dists(maze : &Array2D<char>, start_pos : (usize, usize), start_dir : Direction, dist : &mut Array2D<usize>) {
    
    let mut cur_pos = start_pos;
    let mut next_move = start_dir;
    let mut this_move = start_dir;
    let mut delta : (isize, isize);

    dist[start_pos] = 0;
    
    loop {
        match this_move {
            Direction::Up => {
                delta = (-1, 0);
                match maze[(cur_pos.0 - 1, cur_pos.1)] {
                    'F' => 
                        next_move = Direction::Right,
                    '|' => 
                        next_move = Direction::Up,
                    '7' => 
                        next_move = Direction::Left,
                    'S' =>
                        break,
                    _ => 
                        panic!("Invalid maze character"),
                }
            } Direction::Down => {
                delta = (1, 0);
                match maze[(cur_pos.0 + 1, cur_pos.1)] {
                    'J' => 
                        next_move = Direction::Left,
                    '|' => 
                        next_move = Direction::Down,
                    'L' => 
                        next_move = Direction::Right,
                    'S' =>
                        break,
                    _ => 
                        panic!("Invalid maze character"),
                }
            } Direction::Left => {
                delta = (0, -1);
                match maze[(cur_pos.0, cur_pos.1 - 1)] {
                    'F' => 
                        next_move = Direction::Down,
                    '-' => 
                        next_move = Direction::Left,
                    'L' => 
                        next_move = Direction::Up,
                    'S' =>
                        break,
                    _ => 
                        panic!("Invalid maze character"),
                }
            } Direction::Right => {
                delta = (0, 1);
                match maze[(cur_pos.0, cur_pos.1 + 1)] {
                    'J' => 
                        next_move = Direction::Up,
                    '-' => 
                        next_move = Direction::Right,
                    '7' => 
                        next_move = Direction::Down,
                    'S' =>
                        break,
                    _ =>
                        panic!("Invalid maze character"),
                }
            }
        }
        
        // update current pos
        let old_dist = dist[cur_pos];
        cur_pos = ((cur_pos.0 as isize + delta.0) as usize, (cur_pos.1 as isize + delta.1) as usize);
               
        if dist[cur_pos] != usize::MAX {
            // we've already been here, so we're done
            break;
        }

        dist[cur_pos] = old_dist + 1;
        this_move = next_move;
    }
}

// Given a maze and a starting position, figure out which way to go first
// based on neighboring characters. reverse_search can be used 
fn determine_initial_direction(maze : &Array2D<char>, start_pos : (usize, usize), reverse_search : bool) -> Direction {
    let search_order : Vec<Direction> = if reverse_search {
        vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left]
    } else {
        vec![Direction::Down, Direction::Left, Direction::Up, Direction::Right]
    };

    let mut start_dir : Direction = Direction::Up;

    for dir in search_order {
        println!("Checking dir: {:?} from cur_pos1 {:?}", dir, start_pos);
        if start_pos.0 > 0 && dir == Direction::Up && (
                maze[(start_pos.0 - 1, start_pos.1)] == '|' || 
                maze[(start_pos.0 - 1, start_pos.1)] == 'F' ||
                maze[(start_pos.0 - 1, start_pos.1)] == '7') {
            start_dir = Direction::Up;
            break;
        } else if start_pos.0 < maze.num_rows() && dir == Direction::Down && (
                maze[(start_pos.0 + 1, start_pos.1)] == '|' ||
                maze[(start_pos.0 + 1, start_pos.1)] == 'J' ||
                maze[(start_pos.0 + 1, start_pos.1)] == 'L') {
            start_dir = Direction::Down;
            break;
        } else if start_pos.1 > 0 && dir == Direction::Left && (
                maze[(start_pos.0, start_pos.1 - 1)] == '-' ||
                maze[(start_pos.0, start_pos.1 - 1)] == 'F' ||
                maze[(start_pos.0, start_pos.1 - 1)] == 'L') {
            start_dir = Direction::Left;
            break;
        } else if start_pos.1 < maze.num_columns() && dir == Direction::Right && (
                maze[(start_pos.0, start_pos.1 + 1)] == '-' ||
                maze[(start_pos.0, start_pos.1 + 1)] == 'J' ||
                maze[(start_pos.0, start_pos.1 + 1)] == '7'){
            start_dir = Direction::Right;
            break;
        }
    }

    return start_dir;
}

fn print_maze(maze : &Array2D<char>) {
    for row in maze.rows_iter() {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
}

// Flood fill algorithm to fill in all the 'o' characters we can reach
fn flood_fill (maze : &mut Array2D<char>, nodei : (isize, isize)) {
    if nodei.0 < 0 || nodei.0 >= maze.num_rows() as isize || nodei.1 < 0 || nodei.1 >= maze.num_columns() as isize {
        return;
    }

    let node = (nodei.0 as usize, nodei.1 as usize);

    if maze[node] == '?' {
        maze[node] = 'O';
    } else if maze[node] == 'o' {
        maze[node] = '.';   // mark as visited
        flood_fill(maze, (node.0 as isize - 1, node.1 as isize));
        flood_fill(maze, (node.0 as isize + 1, node.1 as isize));
        flood_fill(maze, (node.0 as isize, node.1 as isize - 1));
        flood_fill(maze, (node.0 as isize, node.1 as isize + 1));
    } else {
        return;
    }
}

fn main() {
    let mut start_pos = (0, 0);

    let maze_size = (140, 140);  // input.txt
    let maze = load_maze("input.txt", maze_size.0, maze_size.1, &mut start_pos);
    
    let mut dist_1: Array2D<usize> = Array2D::filled_with(usize::MAX, maze_size.0, maze_size.1);
    let mut dist_2: Array2D<usize> = Array2D::filled_with(usize::MAX, maze_size.0, maze_size.1);
    
    // iterate through maze from start_pos, traversing both directions away from the start and looping back to the start. 
    // We'll fill in both dist_1 and dist_2 with the distances to all reachable points, where they have the same value
    // for the same point, that's the intersection and the furthest point.

    // first, fill in dist_1
    let start_dir_1 = determine_initial_direction(&maze, start_pos, false);
    fill_dists(&maze, start_pos, start_dir_1, &mut dist_1);

    // now, fill in dist_2
    let start_dir_2 = determine_initial_direction(&maze, start_pos, true);
    fill_dists(&maze, start_pos, start_dir_2, &mut dist_2);

    // find intersection
    let mut max_dist = 0;

    'outer: for i in 0..maze_size.0 {
        for j in 0..maze_size.1 {
            if dist_1[(i, j)] == dist_2[(i, j)] && dist_1[(i, j)] != 0 && dist_1[(i, j)] != usize::MAX {
                max_dist = dist_1[(i, j)];
                break 'outer;
            }
        }
    }

    println!("Part 1 answer (dist): {}", max_dist);

    // we'll visualize by marking parts of the loop with the maze, then fill in everything else as 'O'
    let mut area: Array2D<char> = Array2D::filled_with('O', maze_size.0, maze_size.1);

    for i in 0..maze_size.0 {
        for j in 0..maze_size.1 {
            if dist_1[(i, j)] == usize::MAX {
                area[(i, j)] = 'O';
            } else {
                area[(i, j)] = maze[(i, j)];
            }
        }
    }

    println!("Original loop:");
    print_maze(&area);

    // we're going to expand the map now to allow for a fill algorithm that reaches "inside the cracks".
    let areax_size = (maze_size.0 * 3, maze_size.1 * 3);
    let mut areax: Array2D<char> = Array2D::filled_with('O', areax_size.0 , areax_size.1);
    
    for i in 0..maze_size.0 {
        for j in 0..maze_size.1 {
            match area[(i, j)] {
                'O' => {
                    // row, col
                    areax[(i * 3,     j * 3)]     = 'o';
                    areax[(i * 3,     j * 3 + 1)] = 'o';
                    areax[(i * 3,     j * 3 + 2)] = 'o';
                    areax[(i * 3 + 1, j * 3)]     = 'o';
                    areax[(i * 3 + 1, j * 3 + 1)] = '?';
                    areax[(i * 3 + 1, j * 3 + 2)] = 'o';
                    areax[(i * 3 + 2, j * 3)]     = 'o';
                    areax[(i * 3 + 2, j * 3 + 1)] = 'o';
                    areax[(i * 3 + 2, j * 3 + 2)] = 'o';
                },
                'S' => {
                    areax[(i * 3,     j * 3)]     = 'o';
                    areax[(i * 3,     j * 3 + 1)] = 'S';
                    areax[(i * 3,     j * 3 + 2)] = 'o';
                    areax[(i * 3 + 1, j * 3)]     = 'S';
                    areax[(i * 3 + 1, j * 3 + 1)] = 'S';
                    areax[(i * 3 + 1, j * 3 + 2)] = 'S';
                    areax[(i * 3 + 2, j * 3)]     = 'o';
                    areax[(i * 3 + 2, j * 3 + 1)] = 'S';
                    areax[(i * 3 + 2, j * 3 + 2)] = 'o';                    
                },
                'F' => {
                    areax[(i * 3,     j * 3)]     = 'o';
                    areax[(i * 3,     j * 3 + 1)] = 'o';
                    areax[(i * 3,     j * 3 + 2)] = 'o';
                    areax[(i * 3 + 1, j * 3)]     = 'o';
                    areax[(i * 3 + 1, j * 3 + 1)] = 'F';
                    areax[(i * 3 + 1, j * 3 + 2)] = '-';
                    areax[(i * 3 + 2, j * 3)]     = 'o';
                    areax[(i * 3 + 2, j * 3 + 1)] = '|';
                    areax[(i * 3 + 2, j * 3 + 2)] = 'o';
                },
                'J' => {
                    areax[(i * 3,     j * 3)]     = 'o';
                    areax[(i * 3,     j * 3 + 1)] = '|';
                    areax[(i * 3,     j * 3 + 2)] = 'o';
                    areax[(i * 3 + 1, j * 3)]     = '-';
                    areax[(i * 3 + 1, j * 3 + 1)] = 'J';
                    areax[(i * 3 + 1, j * 3 + 2)] = 'o';
                    areax[(i * 3 + 2, j * 3)]     = 'o';
                    areax[(i * 3 + 2, j * 3 + 1)] = 'o';
                    areax[(i * 3 + 2, j * 3 + 2)] = 'o';
                },
                'L' => {
                    areax[(i * 3,     j * 3)]     = 'o';
                    areax[(i * 3,     j * 3 + 1)] = '|';
                    areax[(i * 3,     j * 3 + 2)] = 'o';
                    areax[(i * 3 + 1, j * 3)]     = 'o';
                    areax[(i * 3 + 1, j * 3 + 1)] = 'L';
                    areax[(i * 3 + 1, j * 3 + 2)] = '-';
                    areax[(i * 3 + 2, j * 3)]     = 'o';
                    areax[(i * 3 + 2, j * 3 + 1)] = 'o';
                    areax[(i * 3 + 2, j * 3 + 2)] = 'o';
                },
                '|' => {
                    areax[(i * 3,     j * 3)]     = 'o';
                    areax[(i * 3,     j * 3 + 1)] = '|';
                    areax[(i * 3,     j * 3 + 2)] = 'o';
                    areax[(i * 3 + 1, j * 3)]     = 'o';
                    areax[(i * 3 + 1, j * 3 + 1)] = '|';
                    areax[(i * 3 + 1, j * 3 + 2)] = 'o';
                    areax[(i * 3 + 2, j * 3)]     = 'o';
                    areax[(i * 3 + 2, j * 3 + 1)] = '|';
                    areax[(i * 3 + 2, j * 3 + 2)] = 'o';
                },
                '-' => {
                    areax[(i * 3,     j * 3)]     = 'o';
                    areax[(i * 3,     j * 3 + 1)] = 'o';
                    areax[(i * 3,     j * 3 + 2)] = 'o';
                    areax[(i * 3 + 1, j * 3)]     = '-';
                    areax[(i * 3 + 1, j * 3 + 1)] = '-';
                    areax[(i * 3 + 1, j * 3 + 2)] = '-';
                    areax[(i * 3 + 2, j * 3)]     = 'o';
                    areax[(i * 3 + 2, j * 3 + 1)] = 'o';
                    areax[(i * 3 + 2, j * 3 + 2)] = 'o';
                },
                '7' => {
                    areax[(i * 3,     j * 3)]     = 'o';
                    areax[(i * 3,     j * 3 + 1)] = 'o';
                    areax[(i * 3,     j * 3 + 2)] = 'o';
                    areax[(i * 3 + 1, j * 3)]     = '-';
                    areax[(i * 3 + 1, j * 3 + 1)] = '7';
                    areax[(i * 3 + 1, j * 3 + 2)] = 'o';
                    areax[(i * 3 + 2, j * 3)]     = 'o';
                    areax[(i * 3 + 2, j * 3 + 1)] = '|';
                    areax[(i * 3 + 2, j * 3 + 2)] = 'o';
                },
                _ => {
                    panic!("Invalid area maze character");
                }
            }
        }
    }
    
    println!("Expanded loop map:");
    print_maze(&areax);

    // Now run a fill algorithm. We're going to identify all the 'o' characters we can reach.
    // If we can reach a '?' then we'll replace it with an 'O' to represent an original O outside
    // of the map. The remaining number of question marks were other non-loop empty spaces,
    // and therefore are the inside area.
    flood_fill(&mut areax, (0, 0));

    println!("Filled loop map:");
    print_maze(&areax);

    let mut area_count = 0;
    for i in 0..areax_size.0 {
        for j in 0..areax_size.1 {
            if areax[(i, j)] == '?' {
                area_count += 1;
            }
        }
    }
    println!("Part 2 answer (area): {}", area_count);
}
