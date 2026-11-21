use std::io;

fn build_prefix_sum(nums : &Vec<i64>) -> Vec<i64> {
   let mut result : Vec<i64> = Vec::with_capacity(nums.len()+1);
   result.push(0);
   for num in nums.iter() {
       result.push(result.last().unwrap() + num);
   }
   result
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");

    let (size, request_count) = input.trim().split_once(' ').unwrap();
    let size : usize = size.parse().unwrap();
    let request_count : u32 = request_count.parse().unwrap();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");
    
    let nums : Vec<i64> = input
        .split_whitespace()
        .take(size)
        .map(|c| c.parse().expect("not a number"))
        .collect();

    let prefix_sum : Vec<i64> = build_prefix_sum(&nums);
    for _ in 0..request_count {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("input error");

        let (l, r) = input.trim().split_once(' ').unwrap();
        let l : usize = l.parse().unwrap();
        let r : usize = r.parse().unwrap();

        println!("{}", prefix_sum[r]-prefix_sum[l-1]);
    }
}
