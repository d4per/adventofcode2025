use std::cmp::max;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use counter::Counter;
use itertools::Itertools;
use range_compare;
use range_set::RangeSet;

fn main() {

    let raw_lines = include_str!("input.txt")
        .split("\n")
        .collect_vec();

    let mut iter = raw_lines.iter();

    let mut ranges = Vec::<RangeInclusive<u64>>::new();
    while true {
        let line = iter.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut range_iter = line.split("-").into_iter();
        let lower = range_iter.next().unwrap().parse::<u64>().unwrap();
        let upper = range_iter.last().unwrap().parse::<u64>().unwrap();
        let range = lower ..= upper;
        ranges.push(range);
    }

    let mut ingredients = Vec::<u64>::new();
    while true {
        let line = iter.next();
        if line.is_none() {
            break;
        }
        let ingredient = line.unwrap().parse::<u64>().unwrap();
        ingredients.push(ingredient);
    }

    let mut is_fresh_count = 0;
    for ingredient in ingredients.iter() {
        let mut is_fresh = false;
        for range in &ranges {
            if range.contains(&ingredient) {
                is_fresh = true;
                break;
            }
        }
        if(is_fresh) {
            is_fresh_count += 1;
        }
    }

    println!("{}", is_fresh_count);

    //part 2

    let mut range_set = RangeSet::<[RangeInclusive<u64>; 1]>::new();
    for range in &ranges {
        let mut other = RangeSet::<[RangeInclusive<u64>; 1]>::new();
        other.insert_range(range.clone());
        range_set = range_set.union(&other);
    }
    println!("{}", range_set.len());

}