use std::fs;

fn main() {

    let input = fs::read_to_string("./input_d4.txt")
        .expect("Plz be something in this file");
    let data = input.split("\n").collect::<Vec<&str>>();

    let mut enclosed:u16 = 0;
    let mut overlap:u16 = 0;

    for line in data{
        let pairs = [
            line.split(",").collect::<Vec<&str>>()[0]
                .split("-").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),

            line.split(",").collect::<Vec<&str>>()[0] // I'm sure there is a better way of doing this with a foreach or something but I don't know rust
                .split("-").collect::<Vec<&str>>()[1].parse::<i32>().unwrap(),

            line.split(",").collect::<Vec<&str>>()[1]
                .split("-").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),

            line.split(",").collect::<Vec<&str>>()[1]
                .split("-").collect::<Vec<&str>>()[1].parse::<i32>().unwrap()
        ];

        enclosed +=(pairs[0] >= pairs[2] && pairs[1] <= pairs[3]) || (pairs[2] >= pairs[0] && pairs[3] <= pairs[1]);
        overlap  +=(pairs[0] >= pairs[2] && pairs[0] <= pairs[3]) || (pairs[1] >= pairs[2] && pairs[1] <= pairs[3]) || (pairs[2] >= pairs[0] && pairs[2] <= pairs[1]) || (pairs[3] >= pairs[0] && pairs[3] <= pairs[1])

    }
    println!("Part 1: {}", enclosed);
    println!("Part 2: {}", overlap);
}