use std::io;

fn main() {
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
    
    let nums : Vec<u16> = input
        .split_whitespace()
        .take(size)
        .map(|c| c.parse().expect("not a number"))
        .collect();

    let mut stack : Vec<u16> = Vec::new();
    let mut to_impasse : u16 = 0;
    let mut from_impasse : u16 = 0;
    let mut last_number : u16 = 0;
    let mut answer : String = String::new();
    
    for num in nums {
        to_impasse += 1;
        if !stack.is_empty() && num > *stack.last().unwrap() {
            println!("0\n");
            return;
        }

        if num == last_number + 1 {
            answer.push_str(&format!("1 {to_impasse}\n"));
            to_impasse = 0;
            last_number += 1;
            from_impasse += 1;
        } else {
            stack.push(num);
        }

        while stack.last() == Some(&(last_number + 1)) {
            from_impasse += 1;
            last_number += 1;
            stack.pop();
        }

        if from_impasse != 0 {
            answer.push_str(&format!("2 {from_impasse}\n"));
            from_impasse = 0;
        }
    }

    println!("{answer}");
}
