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

    let mut tot_sum = 0u64;
    let mut column_numbers = Vec::<u64>::new();

    for i in (0..raw_lines[0].len()).rev() {
        let mut is_empty_column = true;
        let mut current_num = 0u64;
        for j in 0..raw_lines.len() {
            let current_char: char = raw_lines[j].chars().nth(i).unwrap();
            if current_char != ' ' {
                is_empty_column = false;
            }
            if current_char.is_numeric() {
                current_num = current_num * 10 + (current_char as u64 - '0' as u64);
            } else if current_char == '*' {
                for num in &column_numbers {
                    current_num = current_num * num;
                }
                tot_sum += current_num;
            } else if current_char == '+' {
                for num in &column_numbers {
                    current_num = current_num + num;
                }
                tot_sum += current_num;
            }
        }
        if is_empty_column {
            column_numbers.clear();
        } else {
            column_numbers.push(current_num);
        }
    }
    println!("tot_sum {}", tot_sum);
    //3649797
}

fn part1() {

    let raw_lines = include_str!("input.txt")
        .split("\n")
        .collect_vec();

    let row1 = read_numbers(raw_lines[0]);
    let row2 = read_numbers(raw_lines[1]);
    let row3 = read_numbers(raw_lines[2]);
    let row4 = read_numbers(raw_lines[3]);
    let operators = read_operators(raw_lines[4]);
    let lines = [&row1, &row2, &row3, &row4];


    let mut sum_answers: u64 = 0;
    for i in 0..row1.len() {
        let operator = operators[i];
        let mut a: u64 = if operator == '+' { 0 } else { 1 };
        for line in lines {
            let num = line[i];
            if operator == '+' {
                a += num;
            } else {
                a *= num;
            }
        }
        sum_answers += a;
    }
    println!("aa {}",sum_answers);
}

fn read_numbers(line: &str) -> Vec<u64> {
    let mut numbers: Vec<u64> = Vec::new();
    line.split(" ").into_iter().for_each(|s| {
        if !s.is_empty() {
            numbers.push(s.parse::<u64>().unwrap());
        }
    });
    numbers
}
fn read_operators(line: &str) -> Vec<char> {
    let mut numbers: Vec<char> = Vec::new();
    line.split(" ").into_iter().for_each(|s| {
        if !s.is_empty() {
            numbers.push(s.chars().next().unwrap());
        }
    });
    numbers
}