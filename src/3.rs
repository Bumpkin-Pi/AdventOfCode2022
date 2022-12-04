use std::fs;

fn main() {

    let input = fs::read_to_string("./input_d3.txt")
        .expect("Plz be something in this file");
    let data = input.split("\n").collect::<Vec<&str>>();

    let mut total_value:u32 = 0;

    for line in data{
        let first_half = &line[..(line.chars().count()/2)];
        let second_half = &line[(line.chars().count()/2)..];


        for char in first_half.chars(){
            if second_half.contains(char){
                let value:u32 = if 97 <= (char as u32) && (char as u32) <= 122 { char as u32 - 96 }else{ char as u32 - 38 };
                total_value += value;
                break;
            }
        }
    }
    println!("{}", total_value)
}