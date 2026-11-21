use std::io;

fn main() {
    let mut counts : [usize;101] = [0;101];
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");

    let size : usize = input
        .trim()
        .parse()
        .expect("not a number");

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("input 2 error");
    
    let nums : Vec<usize> = input
        .split_whitespace()
        .take(size)
        .map(|c| c.parse().expect("not a number"))
        .collect();

    for num in nums {
        counts[num] += 1;
    }

    let mut answer = String::new();
    for i in 0..=100 {
        answer.push_str(&format!("{i} ").repeat(counts[i]));
    }
    answer.pop();

    println!("{answer}");

}
