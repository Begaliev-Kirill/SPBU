use std::io;
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");

    let n : usize = input
        .trim()
        .parse()
        .expect("not a number");

    let mut queue : VecDeque<i32> = VecDeque::new();
    let mut min_queue : VecDeque<i32> = VecDeque::new();

    for _ in 0..n {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("input 2 error");

        let mut input = input.trim().split_whitespace();
        let command = input.next().expect("trust me") == "+";
        if command {
            let number : i32 = input.next().expect("trust me").parse().expect("trust me again");
            queue.push_front(number);

            loop {
                if let Some(min) = min_queue.back() {
                    if *min > number {
                        min_queue.pop_back();
                        continue;
                    } 
                    else {
                        break;
                    }
                }
                break;
            }

            min_queue.push_back(number);
            println!("{}", min_queue.front().unwrap_or(&-1));
        } else {
            let pop : i32 = queue.pop_back().expect("trust me trust me trust me again");
            
            if *min_queue.front().unwrap() == pop {
                min_queue.pop_front();
            }
            println!("{}", min_queue.front().unwrap_or(&-1));
        }
    }
}
