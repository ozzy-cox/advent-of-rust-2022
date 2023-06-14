use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum HandResult {
    Lose,
    Draw,
    Win,
}

fn main() {
    let file = File::open("input.txt").expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut myscore: i32 = 0;

    for line in buf_reader.lines() {
        let lineresult = line.unwrap();
        let mut parts = lineresult.split_whitespace();
        let op_hand = match parts.next() {
            Some("A") => Hand::Rock,
            Some("B") => Hand::Paper,
            Some("C") => Hand::Scissors,
            None => panic!(),
            Some(_) => panic!(),
        };
        let result = match parts.next() {
            Some("X") => HandResult::Lose,
            Some("Y") => HandResult::Draw,
            Some("Z") => HandResult::Win,
            None => panic!(),
            Some(_) => panic!(),
        };

        myscore += match op_hand {
            Hand::Rock => match result {
                HandResult::Lose => 3,
                HandResult::Draw => 4,
                HandResult::Win => 8
            },
            Hand::Paper => match result {
                HandResult::Lose => 1,
                HandResult::Draw => 5,
                HandResult::Win => 9,
            },

            Hand::Scissors => match result {
                HandResult::Lose => 2,
                HandResult::Draw => 6,
                HandResult::Win => 7
            },
        }
    }

    println!("My score if I follow the second strategy guide {myscore}")
}
