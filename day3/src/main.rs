use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
fn main() {
    let file = File::open("input.txt").expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut sum: usize = 0;
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let alphabet_map: HashMap<char, usize> =
        HashMap::from_iter(alphabet.chars().enumerate().map(|(i, c)| (c, i + 1)));

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let mut first_compartment_items: HashSet<char> =
            HashSet::from_iter(line[..line.len() / 2].chars());

        for i in 0..line.len() / 2 {
            first_compartment_items.insert(line.chars().nth(i).unwrap());
        }

        for i in line.len() / 2..line.len() {
            let char = line.chars().nth(i).unwrap();
            if first_compartment_items.contains(&char) {
                sum += alphabet_map.get(&char).unwrap();
                break;
            }
        }
    }
    println!("Sum is {sum}");
}
