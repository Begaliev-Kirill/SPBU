use std::io;
use std::cmp;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("input 2 error");
    
    let mut iter = input.split_whitespace();
    let mut count: u64 = iter.next().unwrap().parse().unwrap();
    let speed_1: u64 = iter.next().unwrap().parse().unwrap();
    let speed_2: u64 = iter.next().unwrap().parse().unwrap();
    let mut answer : u64 = 0;

    let ls = cmp::min(speed_1, speed_2);
    let ms = cmp::max(speed_1, speed_2);
    
    answer += ls;
    count -= 1;

    let t : u64 = count*ls / (ls + ms);
    let mut best_one = cmp::max(t*ms, (count-t)*ls);
    
    if t + 1 <= count {
        best_one = cmp::min(best_one, cmp::max((t+1)*ms, (count-t-1)*ls));
    }

    answer += best_one;
    
    println!("{answer}");
}
