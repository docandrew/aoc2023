use std::fs;
use array2d::Array2D;

// Load from a file into a 2-d matrix. Return the size of the matrix as 
// (num_rows, num_columns)
fn load_mat(filename : &str, mat_size : &mut (usize, usize)) -> Array2D<char> {
    let contents = fs::read_to_string(filename).
    expect("Something went wrong reading the file");

    // determine number of rows
    mat_size.0 = 1;
    for c in contents.chars() {
        if c == '\n' {
            mat_size.0 += 1;
        }
    }

    // determine number of columns
    mat_size.1 = 0;
    for line in contents.lines() {
        if line.len() > mat_size.1 {
            mat_size.1 = line.len();
        }
    }

    let mut mat = Array2D::filled_with('?', mat_size.0, mat_size.1);
    let mut row: usize = 0;
    let mut col: usize = 0;

    for c in contents.chars() {
        if c == '\n' {
            row += 1;
            col = 0;           
        } else {
            mat[(row, col)] = c;
            col += 1;
        }
    }

    return mat;
}

#[allow(dead_code)]
fn print_matrix(mat: &Array2D<char>) {
    for i in 0..mat.num_rows() {
        for j in 0..mat.num_columns() {
            print!("{}", mat[(i, j)]);
        }
        println!("");
    }
}

// manhattan distance between two points
// note the subtract 1 since the first step away from the star is not counted
fn manhattan_dist(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
    return (pos1.0 as i32 - pos2.0 as i32).abs() as usize + (pos1.1 as i32 - pos2.1 as i32).abs() as usize;
}

fn main() {
    let mut gal0_size = (0, 0);
    let gal0 = load_mat("input.txt", &mut gal0_size);

    // Make lists of empty rows and columns.
    let mut empty_rows: Vec<bool> = vec![true; gal0_size.0];
    let mut empty_cols: Vec<bool> = vec![true; gal0_size.1];

    for i in 0..gal0_size.0 {
        for j in 0..gal0_size.1 {
            if gal0[(i, j)] == '#' {
                // mark as not empty
                empty_rows[i] = false;
                empty_cols[j] = false;
            }
        }
    }

    // store stars as a list of (row, col) tuples
    let mut stars: Vec<(usize, usize)> = Vec::new();

    for i in 0..gal0_size.0 {
        for j in 0..gal0_size.1 {
            if gal0[(i, j)] == '#' {
                stars.push((i, j));
            }
        }
    }

    let mut sum = 0;
    let mut sum_2 = 0;

    for i in 0..stars.len() {
        for j in i+1..stars.len() {
            let star_i = stars[i];
            let star_j = stars[j];

            // determine number of empty rows and cols between these stars
            //
            // N.B. I'm glad I did it this way and not actually expanding the matrix
            // (my original plan but this seemed easier for part 1 - it paid off for part 2!)
            let (sr, er) = if star_i.0 < star_j.0 { (star_i.0, star_j.0) } else { (star_j.0, star_i.0) };
            let (sc, ec) = if star_i.1 < star_j.1 { (star_i.1, star_j.1) } else { (star_j.1, star_i.1) };
            let empty_rows_between : i32 = empty_rows[sr..er].iter().map(|x| if *x { 1 } else { 0 }).sum();
            let empty_cols_between : i32 = empty_cols[sc..ec].iter().map(|x| if *x { 1 } else { 0 }).sum();

            let dist = manhattan_dist(star_i, star_j) +
                                empty_rows_between as usize + 
                                empty_cols_between as usize;
            
            let big_dist = manhattan_dist(star_i, star_j) +
                                empty_rows_between as usize * 999_999 + 
                                empty_cols_between as usize * 999_999;
            
            sum += dist;
            sum_2 += big_dist;
        }
    }

    println!("Part 1 answer (sum of dists): {}", sum);
    println!("Part 2 answer (sum of dists with 1_000_000 multiplier): {}", sum_2);
}
