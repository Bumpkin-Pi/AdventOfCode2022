use std::fs;
use std::collections::HashMap;


fn main() {

    let states = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);

    let input = fs::read_to_string("./input.txt")
        .expect("Plz be something in this file");

    let data = input.split("\n").collect::<Vec<&str>>();

    let mut score = 0;

    for line in data{
        score += states[line];
    }

    println!("{}", score)
}