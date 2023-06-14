use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
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
        let my_hand = match parts.next() {
            Some("X") => Hand::Rock,
            Some("Y") => Hand::Paper,
            Some("Z") => Hand::Scissors,
            None => panic!(),
            Some(_) => panic!(),
        };

        myscore += match op_hand {
            Hand::Rock => match my_hand {
                Hand::Rock => 4,
                Hand::Paper => 8,
                Hand::Scissors => 3,
            },
            Hand::Paper => match my_hand {
                Hand::Rock => 1,
                Hand::Paper => 5,
                Hand::Scissors => 9,
            },

            Hand::Scissors => match my_hand {
                Hand::Rock => 7,
                Hand::Paper => 2,
                Hand::Scissors => 6,
            },
        }
    }

    println!("My score if I follow the strategy guide {myscore}")
}
