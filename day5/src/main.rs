use std::fs;

fn main() {
    part1_2();
}

// src start, src end, dest start, dest end
#[derive(Debug)]
struct Mapping(u64, u64, u64, u64);

// Map source value to destination value based on mappings.
fn s2d(i : u64, ms : &Vec<Mapping>) -> u64 {
    // first, find mapping where i is between
    // the source start and end
    for m in ms {
        if i >= m.0 && i <= m.1 {
            // now, find the destination value
            // by subtracting the source start
            // from i, and adding that to the
            // destination start
            return m.2 + (i - m.0);
        }
    }

    // if we get here, we didn't find a mapping
    // so return the same number, since there's always an assumed 1-to-1 mapping.
    return i;
}

// Store the beginning and end of values.
#[derive(Debug)]
#[derive(Clone)]
struct MapRange(u64, u64);

// Split a range of values into smaller ranges based on mappings.
fn split_range (i : MapRange, ms : &Vec<Mapping>) -> Vec<MapRange> {
    let mut result : Vec<MapRange> = Vec::new();

    for m in ms {
        if i.0 >= m.0 && i.1 <= m.1 {
            // range contained entirely in a mapping, no need to split
            result.push(MapRange(i.0, i.1));
            break;
        } else if i.1 < m.0 || i.0 > m.1 {
            // range does not overlap in this mapping, check next one.
            continue;
        } else if i.0 < m.0 && i.1 <= m.1 {
            // range overlaps with beginning of mapping, split into two
            // ranges, one before the mapping, and one inside the mapping.
            // recursively check the left range for more mappings.
            let mut left_range = split_range(MapRange(i.0, m.0 - 1), ms);
            result.append(&mut left_range);     // before mapping
            result.push(MapRange(m.0, i.1));    // inside mapping
        } else if i.0 >= m.0 && i.1 > m.1 {
            // range overlaps with end of mapping, split into two 
            // ranges, one inside the mapping, and one after the mapping.
            // recursively check the right range for more mappings.
            let mut right_range = split_range(MapRange(m.1 + 1, i.1), ms);
            result.append(&mut right_range);    // after mapping
            result.push(MapRange(i.0, m.1));    // inside mapping
        } else if i.0 < m.0 && i.1 > m.1 {
            // range overlaps with entire mapping, split into three
            // ranges, one before the mapping, one inside the mapping,
            // and one after the mapping.
            // recursively check the left and right ranges for more mappings.
            let mut left_range = split_range(MapRange(i.0, m.0 - 1), ms);
            result.append(&mut left_range);     // before mapping
            result.push(MapRange(m.0, m.1));    // inside mapping
            let mut right_range = split_range(MapRange(m.1 + 1, i.1), ms);
            result.append(&mut right_range);    // after mapping
        }
    }

    // if no mappings intersect with this range, return the same range.
    if result.len() == 0 {
        result.push(MapRange(i.0, i.1));
    }

    return result;
}

// map source range to dest range based on mappings. Input range is assumed to be
// split such that overlaps with either 1 or 0 mappings.
fn s2ds(i : MapRange, ms : &Vec<Mapping>) -> MapRange {
    for m in ms {
        if i.0 >= m.0 && i.1 <= m.1 {
            // shift input range into destination range by the difference
            // of src and dest start values.
            let diff = m.2 as i64 - m.0 as i64;
            return MapRange((i.0 as i64 + diff) as u64, (i.1 as i64 + diff) as u64);
        }
    }

    // if we get here, we didn't find a mapping so return the same range, 
    // since there's always an assumed 1-to-1 mapping.
    return i;
}

fn part1_2() {   
    let binding = fs::read_to_string("input.txt").
    expect("Something went wrong reading the file");

    let mut lines = binding.split("\n");

    // first line is list of seeds
    let seeds : Vec<u64> = lines.next().unwrap().split(":").skip(1).next().unwrap().
        split_whitespace().map(|x| x.trim().parse::<u64>().unwrap()).collect();

    let mut seed2soil : Vec<Mapping> = Vec::new();
    let mut soil2fert : Vec<Mapping> = Vec::new();
    let mut fert2watr : Vec<Mapping> = Vec::new();
    let mut watr2lite : Vec<Mapping> = Vec::new();
    let mut lite2temp : Vec<Mapping> = Vec::new();
    let mut temp2humi : Vec<Mapping> = Vec::new();
    let mut humi2locn : Vec<Mapping> = Vec::new();

    let mut current_map : &mut Vec<Mapping> = &mut seed2soil;

    // iterate through rest of lines. We'll keep track of which "category"
    // we're in and add mappings to the respective Vec<Mapping>
    for line in lines {
        if line == "" {
            continue;
        } else if line.starts_with("seed") {
            current_map = &mut seed2soil;
        } else if line.starts_with("soil") {
            current_map = &mut soil2fert;
        } else if line.starts_with("fert") {
            current_map = &mut fert2watr;
        } else if line.starts_with("water") {
            current_map = &mut watr2lite;
        } else if line.starts_with("light") {
            current_map = &mut lite2temp;
        } else if line.starts_with("temp") {
            current_map = &mut temp2humi;
        } else if line.starts_with("humi") {
            current_map = &mut humi2locn;
        } else {
            // parse this line into a mapping.
            let mut this_mapping: Mapping = Mapping(0, 0, 0, 0);
            let mut parts = line.split_whitespace().map(|x| x.trim().parse::<u64>().unwrap());
            this_mapping.2 = parts.next().unwrap();
            this_mapping.0 = parts.next().unwrap();
            let len = parts.next().unwrap();
            this_mapping.1 = this_mapping.0 + len;
            this_mapping.3 = this_mapping.2 + len;
            current_map.push(this_mapping);
        }
    }

    let mut min_locn = u64::MAX;

    for seed in seeds.clone() {
        let soil = s2d(seed, &seed2soil);
        let fert = s2d(soil, &soil2fert);
        let watr = s2d(fert, &fert2watr);
        let lite = s2d(watr, &watr2lite);
        let temp = s2d(lite, &lite2temp);
        let humi = s2d(temp, &temp2humi);
        let locn = s2d(humi, &humi2locn);

        if locn < min_locn {
            min_locn = locn;
        }
    }

    println!("Part 1 Answer: {}", min_locn);

    // For part 2 we need to consider ranges of seeds.
    // parse seeds into ranges
    let mut seed_ranges : Vec<MapRange> = Vec::new();
    for pair in seeds.chunks(2) {
        // second number is the _number of values_, so need to take one less.
        seed_ranges.push(MapRange(pair[0], pair[0] + pair[1] - 1));
    }
   
    let mut most_min_locn = u64::MAX;   // smallest location seen so far
    
    for seed_range in seed_ranges {
        let mut soil_ranges : Vec<MapRange> = Vec::new();
        let split_seed = split_range(seed_range, &seed2soil);

        for ss in split_seed {
            soil_ranges.push(s2ds(ss, &seed2soil));
        }
       
        for soil_range in &soil_ranges {
            let mut fert_ranges : Vec<MapRange> = Vec::new();

            let split_soil = split_range(soil_range.clone(), &soil2fert);

            for ss in split_soil {
                fert_ranges.push(s2ds(ss, &soil2fert));
            }

            for fert_range in &fert_ranges {
                let mut watr_ranges : Vec<MapRange> = Vec::new();
                let split_fert = split_range(fert_range.clone(), &fert2watr);

                for ss in split_fert {
                    watr_ranges.push(s2ds(ss, &fert2watr));
                }

                for watr_range in &watr_ranges {
                    let mut lite_ranges : Vec<MapRange> = Vec::new();   
                    let split_watr = split_range(watr_range.clone(), &watr2lite);

                    for ss in split_watr {
                        lite_ranges.push(s2ds(ss, &watr2lite));
                    }

                    for lite_range in &lite_ranges {
                        let mut temp_ranges : Vec<MapRange> = Vec::new();   
                        let split_lite = split_range(lite_range.clone(), &lite2temp);

                        for ss in split_lite {
                            temp_ranges.push(s2ds(ss, &lite2temp));
                        }

                        for temp_range in &temp_ranges {
                            let mut humi_ranges : Vec<MapRange> = Vec::new();
                            let split_temp = split_range(temp_range.clone(), &temp2humi);

                            for ss in split_temp {
                                humi_ranges.push(s2ds(ss, &temp2humi));
                            }

                            for humi_range in &humi_ranges {
                                let mut locn_ranges : Vec<MapRange> = Vec::new();
                                let split_humi = split_range(humi_range.clone(), &humi2locn);

                                for ss in split_humi {
                                    locn_ranges.push(s2ds(ss, &humi2locn));
                                }

                                // now find smallest location in locn_ranges for this seed range.
                                let mut min_locn = u64::MAX;
                                for lr in &locn_ranges {
                                    if lr.0 < min_locn {
                                        min_locn = lr.0;
                                    }
                                }

                                if min_locn < most_min_locn {
                                    most_min_locn = min_locn;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Part 2 Answer: {}", most_min_locn);
}
