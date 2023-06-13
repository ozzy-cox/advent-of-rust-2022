use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
fn main() -> std::io::Result<()> {
    let file = File::open("input.txt").expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut v: Vec<i32> = Vec::new();
    v.push(0);
    let mut idx: usize = 0;
    let mut maxes: Vec<i32> = vec![0, 0, 0];

    for line in buf_reader.lines() {
        let line1 = line.unwrap();
        if "".eq(&line1) {
            idx += 1;
            v.push(0);
        } else {
            v[idx] += line1.parse::<i32>().unwrap();
        }
    }
    let max = v.iter().max().unwrap();

    for calories in v.iter() {
        if calories > &maxes[0] {
            maxes.remove(2);
            maxes.insert(0, *calories);
        } else if calories > &maxes[1] {
            maxes.remove(2);
            maxes.insert(1, *calories);
        } else if calories > &maxes[2] {
            maxes.remove(2);
            maxes.insert(2, *calories);
        }
    }

    let index = v.iter().position(|element| element == max).unwrap();
    println!(
        "{} th elf is the one with the most calories with {} calories",
        index + 1,
        max
    );
    println!("Sum of top 3 elves is {:?}", maxes.iter().sum::<i32>());
    Ok(())
}
