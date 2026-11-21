use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");

    let n : usize = input
        .trim()
        .parse()
        .expect("not a number");

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("input 2 error");
    
    let mut nums : Vec<i32> = input
        .split_whitespace()
        .take(n)
        .map(|c| c.parse().expect("not a number"))
        .collect();
    
    for i in 1..=n {
        if 2*i > n {
            break;
        }

        if nums[i-1] > nums[2*i-1] {
            println!("NO");
            return;
        }

        if 2*i + 1 <= n {
            if nums[i-1] > nums[2*i] {
                println!("NO");
                return;
            }
        }
    }

    println!("YES");
    return;
}
