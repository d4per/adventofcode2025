use counter::Counter;
use itertools::Itertools;

fn main() {

    let raw_map = include_str!("input.txt")
        .split("\n")
        .collect_vec();


    let mut current_rot = 50i64;
    let mut code = 0;
    for t in raw_map {
        if t.trim().is_empty() {
            continue;
        }
        let is_left = t.starts_with("L");
        let number: &u64 = &t[1..t.len()].to_string().parse().unwrap();
        println!("{} {}", is_left, number);


        // part 1
        // if is_left {
        //     current_rot -= number;
        // } else {
        //     current_rot += number;
        // }

        for n in 0..*number {
            if is_left {
                current_rot -= 1;
            } else {
                current_rot += 1;
            }

            current_rot = (current_rot + 100 * 10000) % 100;

            if current_rot == 0 {
                code += 1;
            }
        }
        println!("current_rot {} code {}", current_rot, code);
    }

    println!("code {}", code);

}