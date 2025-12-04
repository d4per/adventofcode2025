use std::cmp::max;
use std::collections::HashSet;
use counter::Counter;
use itertools::Itertools;



fn part1() {

    let map = include_str!("input.txt")
        .split("\n")
        .map(|s| s.chars().collect_vec()).collect_vec();

    let width = map[0].len();
    let height = map.len();

    let mut movable_count: u64 = 0;
    for y in 0i32..height as i32  {
        for x in 0i32..width as i32 {
            if map[y as usize ][x as usize] == '@' {
                let mut neighbours = 0;
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        let xx = x + dx;
                        let yy = y + dy;
                        if !(dx == 0 && dy == 0) && xx >= 0 && xx < width as i32 && yy >= 0 && yy < height as i32 {
                            if map[yy as usize][xx as usize] == '@' {
                                neighbours += 1;
                            }
                        }
                    }
                }
                if neighbours < 4 {
                    movable_count += 1;
                }
            }
        }
    }
    println!("{}", movable_count);
    //359


}

struct Map {
    data: Vec<char>,
    width: usize,
    height: usize,
}




fn main() {
    let mut map = include_str!("input.txt")
        .split("\n")
        .map(|mut s| s.chars().collect_vec()).collect_vec();


    let width = map[0].len();
    let height = map.len();

    let mut removed: u64 = 0;
    while true {
        let movable = find_movable(&map, width, height);
        if movable.is_empty() {
            break;
        }
        for coord in movable {
            map[coord.1][coord.0] = '.';
            removed += 1;
        }
    }
    println!("{}", removed);


}
fn find_movable(map: &Vec<Vec<char>>, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut movable_coordinates = Vec::<(usize, usize)>::new();
    for y in 0i32..height as i32 {
        for x in 0i32..width as i32 {
            if map[y as usize][x as usize] == '@' {
                let mut neighbours = 0;
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        let xx = x + dx;
                        let yy = y + dy;
                        if !(dx == 0 && dy == 0) && xx >= 0 && xx < width as i32 && yy >= 0 && yy < height as i32 {
                            if map[yy as usize][xx as usize] == '@' {
                                neighbours += 1;
                            }
                        }
                    }
                }
                if neighbours < 4 {
                    movable_coordinates.push((x as usize, y as usize));
                }
            }
        }
    }
    movable_coordinates
}