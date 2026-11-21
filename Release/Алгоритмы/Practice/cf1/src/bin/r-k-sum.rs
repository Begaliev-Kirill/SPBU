use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");

    let mut iter = input.split_whitespace();

    let n : usize = iter.next().unwrap().parse().unwrap();
    let k : u64 = iter.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("input 2 error");
    
    let mut a : Vec<i64> = input
        .split_whitespace()
        .take(n)
        .map(|c| c.parse().expect("not a number"))
        .collect();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("input 2 error");
    
    let mut b : Vec<i64> = input
        .split_whitespace()
        .take(n)
        .map(|c| c.parse().expect("not a number"))
        .collect();

    a.sort();
    b.sort();

    let mut l : i64 = a[0] + b[0];
    let mut r : i64 = a[n-1] + b[n-1];
    let mut answer : i64 = l;

    while l <= r {
        let mid : i64 = (l + r) / 2;

        let mut count : u64 = 0;
        let mut b_ptr = n;

        for a_ptr in 0..n {

            while b_ptr > 0 && a[a_ptr] + b[b_ptr-1] > mid {
                b_ptr -= 1;
            }

            count += b_ptr as u64;
            if count >= k {
                break
            }
        }

        if count >= k {
            answer = mid;
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    println!("{answer}");
}
