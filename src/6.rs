use std::fs;



fn main() {

    let input = fs::read_to_string("./input_d6.txt")
        .expect("Plz be something in this file");

    let mut part1complete = false;


    for (i,v) in input.chars().enumerate(){

        if i >= input.chars().count()-4 {
        }else{

            let mut slice = Vec::<char>::new();
            slice.push(input.chars().nth(i).unwrap());

            if !slice.contains(&input.chars().nth(i+1).unwrap()){
                slice.push(input.chars().nth(i+1).unwrap());
            }
            if !slice.contains(&input.chars().nth(i+2).unwrap()){
                slice.push(input.chars().nth(i+2).unwrap());
            }
            if !slice.contains(&input.chars().nth(i+3).unwrap()){
                slice.push(input.chars().nth(i+3).unwrap());
            }/*  //uncomment for Part 2
            if !slice.contains(&input.chars().nth(i+4).unwrap()){
                slice.push(input.chars().nth(i+4).unwrap());
            }
            if !slice.contains(&input.chars().nth(i+5).unwrap()){
                slice.push(input.chars().nth(i+5).unwrap());
            }
            if !slice.contains(&input.chars().nth(i+6).unwrap()){
                slice.push(input.chars().nth(i+6).unwrap());
            }
            if !slice.contains(&input.chars().nth(i+7).unwrap()){
                slice.push(input.chars().nth(i+7).unwrap());
            }
            if !slice.contains(&input.chars().nth(i+8).unwrap()){
                slice.push(input.chars().nth(i+8).unwrap());
            }
            if !slice.contains(&input.chars().nth(i+9).unwrap()){
                slice.push(input.chars().nth(i+9).unwrap());
            }
            if !slice.contains(&input.chars().nth(i+10).unwrap()){
                slice.push(input.chars().nth(i+10).unwrap());
            }
            if !slice.contains(&input.chars().nth(i+11).unwrap()){
                slice.push(input.chars().nth(i+11).unwrap());
            }
            if !slice.contains(&input.chars().nth(i+12).unwrap()){
                slice.push(input.chars().nth(i+12).unwrap());
            }
            if !slice.contains(&input.chars().nth(i+13).unwrap()){
                slice.push(input.chars().nth(i+13).unwrap());
            }*/



            //p1
            if slice.len() == 4 && !part1complete{
                println!("Part 1: {:?}   {}", slice, i+4);
                part1complete = true;
            }
            //p2
            if slice.len() == 14{
                println!("Part 2: {:?}   {}", slice, i+14);
                part1complete = true;
            }



            // if v != input.chars().nth(i+1).unwrap() && v != input.chars().nth(i+2).unwrap() && v != input.chars().nth(i+3).unwrap() && v != input.chars().nth(i+4).unwrap(){
            //     println!("{}   {}", i, v);
            //
            //     // println!("{}{}{}{} {}", input.chars()[i], input.chars()[i+1], input.chars()[i+2], input.chars()[i+4], v)
            // }

        }
    }

}
