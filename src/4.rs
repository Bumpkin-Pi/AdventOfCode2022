use std::fs;

fn main() {

    let input = fs::read_to_string("./input_d4.txt")
        .expect("Plz be something in this file");
    let data = input.split("\n").collect::<Vec<&str>>();

    let mut enclosed:u16 = 0;
    let mut overlap:u16 = 0;


    for line in data{
        let pairs = [
            line.split(",").collect::<Vec<&str>>()[0]   // I'm sure there is a better way of
                .split("-").collect::<Vec<&str>>(),     // doing this with a foreach or something but I don't know rust
            line.split(",").collect::<Vec<&str>>()[1]
                .split("-").collect::<Vec<&str>>()
        ];



        if (pairs[0][0].parse::<i32>().unwrap() >= pairs[1][0].parse::<i32>().unwrap() && pairs[0][1].parse::<i32>().unwrap() <= pairs[1][1].parse::<i32>().unwrap()) || // Yes ik horribly inefficient but i genuinely dont care.
            (pairs[1][0].parse::<i32>().unwrap() >= pairs[0][0].parse::<i32>().unwrap() && pairs[1][1].parse::<i32>().unwrap() <= pairs[0][1].parse::<i32>().unwrap())
        {
            //println!("{}-{},{}-{}", pairs[0][0], pairs[0][1], pairs[1][0], pairs[1][1]);
            enclosed +=1;
        }else{
            //println!("       - {}-{},{}-{}", pairs[0][0], pairs[0][1], pairs[1][0], pairs[1][1])
        }

        if (pairs[0][0].parse::<i32>().unwrap() >= pairs[1][0].parse::<i32>().unwrap() && pairs[0][0].parse::<i32>().unwrap() <= pairs[1][1].parse::<i32>().unwrap()) || // again, yes lazy, but its only 1000 lines so like who really cares.
            (pairs[0][1].parse::<i32>().unwrap() >= pairs[1][0].parse::<i32>().unwrap() && pairs[0][1].parse::<i32>().unwrap() <= pairs[1][1].parse::<i32>().unwrap()) ||
            (pairs[1][0].parse::<i32>().unwrap() >= pairs[0][0].parse::<i32>().unwrap() && pairs[1][0].parse::<i32>().unwrap() <= pairs[0][1].parse::<i32>().unwrap()) ||
            (pairs[1][1].parse::<i32>().unwrap() >= pairs[0][0].parse::<i32>().unwrap() && pairs[1][1].parse::<i32>().unwrap() <= pairs[0][1].parse::<i32>().unwrap())
        {
            // println!("{}-{},{}-{}", pairs[0][0], pairs[0][1], pairs[1][0], pairs[1][1]);
            overlap +=1;
        }else{
            // println!("       - {}-{},{}-{}", pairs[0][0], pairs[0][1], pairs[1][0], pairs[1][1])
        }
    }

    println!("Part 1: {}", enclosed);
    println!("Part 2: {}", overlap);

}