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
    let mut max:i32 = 0;
    let mut max_elf_inv = "";
    let mut max_elf_index = 0;
    let mut pos = 0;
    //println!("{}", info);
    for p in info{
        pos += 1;
        let mut temp:i32 = 0;
        for p2 in p.split("\n"){
            temp += p2.parse::<i32>().unwrap();
        }
        if temp > max {
            max = temp;
            max_elf_inv = p;
            max_elf_index = pos;
        }
    }
    println!("Max: {} kcal", max);
    println!("Elf inv: \n{}", max_elf_inv);
    println!("Elf number {}", max_elf_index)
}
