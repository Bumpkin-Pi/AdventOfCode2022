fn main() {
    let info = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000".split("\n\n");
    let mut max = vec![];
    let mut temp:i32 = 0;
    //let mut max_elf_inv = "";
    //let mut max_elf_index = 0;
    //let mut pos = 0;
    for p in info{
        //pos += 1;
        temp = 0;
        for p2 in p.split("\n"){
            temp += p2.parse::<i32>().unwrap();
        }
        max.push(temp)
    }
    max.sort();
    max.reverse();

    println!("Max: {} kcal", max[0]+max[1]+max[2]);
    println!("Max: {} kcal", max[0]);
    println!("Max: {} kcal", max[1]);
    println!("Max: {} kcal", max[2]);

    //println!("Elf inv: \n{}", max_elf_inv);
    //println!("Elf number {}", max_elf_index);
}
