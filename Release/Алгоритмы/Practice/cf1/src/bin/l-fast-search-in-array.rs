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
    
    let mut nums : Vec<i64> = input
        .split_whitespace()
        .take(size)
        .map(|c| c.parse().expect("not a number"))
        .collect();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("input 2 error");

    let k : usize = input
        .trim()
        .parse()
        .expect("not a number");
    
    nums.push(-1_000_000_001);
    nums.push(1_000_000_001);
    nums.sort();

    let mut answer : Vec<u64> = Vec::with_capacity(k);
    for _ in 0..k {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("input 2 error");

        let mut it = input.split_whitespace();
        let l: i64 = it
            .next()
            .expect("l")
            .parse()
            .expect("not a number");
        let r: i64 = it
            .next()
            .expect("r")
            .parse()
            .expect("not a number");

        let left_index = find_l_index(&nums, l);
        let right_index = find_r_index(&nums, r);

        answer.push((right_index - left_index - 1) as u64);
    }

    let out: Vec<String> = answer
        .iter()
        .map(|x| x.to_string())
        .collect();

    println!("{}", out.join(" "));
}

fn find_r_index(nums : &Vec<i64>, target : i64) -> usize {
    let mut l: usize = 0;
    let mut r: usize = nums.len();
    while l < r {
        let mid = (l + r) / 2;
        if nums[mid] > target {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    l
}

fn find_l_index(nums : &Vec<i64>, target : i64) -> usize {
    let mut l: usize = 0;
    let mut r: usize = nums.len();
    while l < r {
        let mid = (l + r) / 2;
        if nums[mid] >= target {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    l - 1
}
