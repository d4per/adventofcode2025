use std::cmp::max;
use std::collections::HashSet;
use counter::Counter;
use itertools::Itertools;

fn part1() {

    let raw_str = include_str!("input.txt");

    let mut max_digits:usize = 0;

    let ranges = raw_str.split(",").map(|range| {
        let mut parts = range.split("-").into_iter();
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        max_digits = usize::max(max_digits, second.len());
        println!("{:?}-{:?}-{:?}", first, second, range);
        let r = first.parse::<u64>().unwrap()..=second.parse::<u64>().unwrap();
        r
    }).collect_vec();

    println!("{:?}", max_digits);

    let half_digits = ("9".repeat((max_digits + 1) / 2)).parse::<u64>().unwrap();

    let mut sum_in_ranges: u64 = 0;
    for i in 1..=half_digits {
        let repeated = format!("{}",i).repeat(2);
        let repeated_num = repeated.parse::<u64>().unwrap();
        //println!("{:?}",repeated_num);

        let mut in_ranges = false;
        for range in ranges.iter() {
            if range.contains(&repeated_num) {
                in_ranges = true;
                break;
            }
        }
        if in_ranges {
            println!("in ranges: {:?}",repeated_num);
            sum_in_ranges += repeated_num;
        }

    }
    println!("sum_in_ranges: {:?}", sum_in_ranges);

}

fn main() {

    let raw_str = include_str!("input.txt");

    let mut max_digits:usize = 0;

    let ranges = raw_str.split(",").map(|range| {
        let mut parts = range.split("-").into_iter();
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        max_digits = usize::max(max_digits, second.len());
        println!("{:?}-{:?}-{:?}", first, second, range);
        let r = first.parse::<u64>().unwrap()..=second.parse::<u64>().unwrap();
        r
    }).collect_vec();

    println!("{:?}", max_digits);

    let half_digits = ("9".repeat((max_digits + 1) / 2)).parse::<u64>().unwrap();

    let mut sum_in_ranges: u64 = 0;
    let mut already_found: HashSet<u64> = HashSet::new();
    for repeat_times in 2 ..= (max_digits as usize) {

        for i in 1..=half_digits {
            let repeated = format ! ("{}", i).repeat(repeat_times);
            if repeated.len() > max_digits {
                continue;
            }
            let repeated_num = repeated.parse::< u64 > ().unwrap();
            //println!("{:?}",repeated_num);

            let mut in_ranges = false;
            for range in ranges.iter() {
                if range.contains( & repeated_num) {
                    in_ranges = true;
                    break;
                }
            }
            if in_ranges && !already_found.contains(&repeated_num) {
                println ! ("in ranges: {:?}", repeated_num);
                sum_in_ranges += repeated_num;
                already_found.insert(repeated_num);
            }

        }
    }
    println!("sum_in_ranges: {:?}", sum_in_ranges);
    //17318343178

}