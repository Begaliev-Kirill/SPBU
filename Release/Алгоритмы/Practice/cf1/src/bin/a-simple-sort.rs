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
    
    let mut nums : Vec<i32> = input
        .split_whitespace()
        .take(size)
        .map(|c| c.parse().expect("not a number"))
        .collect();

    nums = simple_sort(&mut nums);

    let mut answer = String::new();
    for i in 0..nums.len() {
        answer.push_str(&nums[i].to_string());
        if i != nums.len() {
            answer.push(' ');
        }
    }
    
    println!("{answer}");
}

fn simple_sort(nums : &mut [i32]) -> Vec<i32> {
    let mut nums : Vec<i32> = nums.to_vec();
    let n : usize = nums.len();

    if n <= 2 {
        if n == 2 && nums[0] > nums[1]{nums.swap(0,1);}
        return nums;
    }

    let mid : usize = n/2;
    let (left,right) = nums.split_at_mut(mid);
    let mut left = simple_sort(left);
    let mut right = simple_sort(right);

    let mut result : Vec<i32> = Vec::with_capacity(n);
    let (mut i, mut j) : (usize, usize) = (0,0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    let mut left_tail = left.split_off(i);
    let mut right_tail = right.split_off(j);
    
    result.append(&mut left_tail);
    result.append(&mut right_tail);

    result
}
