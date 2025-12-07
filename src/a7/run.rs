use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use counter::Counter;
use itertools::Itertools;
use range_compare;
use range_set::RangeSet;


fn part1() {

    let raw_lines = include_str!("input.txt")
        .split("\n")
        .collect_vec();

    let s_pos = raw_lines[0].find('S').unwrap();

    let mut beams = HashSet::new();
    beams.insert(s_pos);

    let mut split_count = 0;
    for line in raw_lines[1..].iter() {
        let mut new_beams = HashSet::<usize>::new();
        let split_positions: HashSet<usize> = line.chars().enumerate()
            .filter(|(_, c)| *c == '^')
            .map(|(i, _)| i)
            .collect();

        println!("split_positions: {:?}", split_positions);

        for beam_pos in &beams {
            if split_positions.iter().contains(beam_pos) {
                new_beams.insert(*beam_pos - 1);
                new_beams.insert(*beam_pos + 1);
                split_count += 1;
            } else {
                new_beams.insert(*beam_pos);
            }
        }
        beams = new_beams;
    }
    print!("split_count: {}", split_count);

}

fn main() {
    let raw_lines = include_str!("input.txt")
        .split("\n")
        .collect_vec();

    let s_pos = raw_lines[0].find('S').unwrap();
    let mut memoization = HashMap::new();
    let num_pos = find_positions(&raw_lines, 1, s_pos, &mut memoization);

    println!("num_pos: {:?}", num_pos);
}


fn find_positions(lines: &[&str], line_index: usize, start_position: usize, memoization: &mut HashMap<(usize, usize), u64>) -> u64 {
    if lines.len() <= line_index {
        return 1;
    }
    match memoization.get(&(line_index,start_position)) {
        None => { }//continue
        Some(count) => { return *count; }
    }

    let mut count = 0u64;
    if lines[line_index].chars().nth(start_position).unwrap() == '^' {
        count += find_positions(lines, line_index + 1, start_position - 1, memoization);
        count += find_positions(lines, line_index + 1, start_position + 1, memoization);
    } else {
        count += find_positions(lines, line_index + 1, start_position, memoization);
    }

    memoization.insert((line_index, start_position), count);
    count
}