
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");

    let count : usize = input
        .trim()
        .parse()
        .expect("not a number");
    
    let mut strings : Vec<String> = Vec::with_capacity(count);
    let mut answer : String = String::new();
    
    for _ in 0..count {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("input error");

        let trimed_string : String = input.trim().to_string();
        
        if trimed_string.matches('/').count() == 0 {
            answer.push_str(&format!("{trimed_string}\n"));   
        } else {
            strings.push(input.trim().to_string());
        }
    }
    strings.sort();

    for string in strings {
        answer.push_str(&"  ".repeat(string.matches('/').count()));
        if let Some((_, tail)) = string.rsplit_once('/') {
            answer.push_str(tail);
            answer.push_str(&"\n");
        }
    }

    println!("{answer}");
}
