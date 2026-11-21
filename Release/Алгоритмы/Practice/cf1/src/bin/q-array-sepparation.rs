use std::io;
use std::cmp;
use std::cmp::Ordering;

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
    
    let nums : Vec<u64> = input
        .split_whitespace()
        .take(n)
        .map(|c| c.parse().expect("not a number"))
        .collect();

    let mut l : u64 = *nums.iter().max().unwrap();
    let mut r : u64 = nums.iter().sum::<u64>();

    while l < r {
        let mid : u64 = (l+r)/2;
        let result = check(&nums, mid, k);

        match result {
            Ordering::Less => r = mid,
            Ordering::Greater => l = mid + 1,
            Ordering::Equal => r = mid, 
        }
    }

    println!("{l}");
}

fn check(nums : &Vec<u64>, target : u64, k : u64) -> cmp::Ordering {
    let mut segs : u64 = 1;
    let mut sum : u64 = 0;

    for num in nums.iter() {
        sum += *num;
        if sum > target {
            segs += 1;
            sum = *num;
        }
    }

    return segs.cmp(&k);
}
