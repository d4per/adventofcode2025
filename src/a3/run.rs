use std::cmp::max;
use std::collections::HashSet;
use counter::Counter;
use itertools::Itertools;



fn main() {

    let raw_lines = include_str!("input.txt")
        .split("\n")
        .collect_vec();


    let mut sum_numbers = 0;

    for line in &raw_lines {
        let mut start_index = 0usize;
        let mut result_number = Vec::<char>::new();
        for digit in (0..12).rev() {
            println!("digit: {}",digit);
            let mut sub_list = line[start_index..line.len() - (digit)].chars().collect_vec();
            let sub_list_org = sub_list.clone();
            sub_list.sort_unstable();
            sub_list.reverse();
            println!("aa {:?}", sub_list);
            let (mut best_index,_) = sub_list_org.iter().find_position(|c| *c == &sub_list[0]).unwrap();
            best_index += start_index;
            println!("{}", best_index);
            start_index = best_index + 1;
            result_number.push(sub_list[0]);
        }
        println!("line number {}", result_number.iter().collect::<String>());

        sum_numbers += result_number.iter().collect::<String>().parse::<u64>().unwrap();
    }

    println!("tot: {}", sum_numbers);
    //17552
    //175553391191293

}