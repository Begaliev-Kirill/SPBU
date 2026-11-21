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

    let mut answer : i64 = merge_sort(&mut nums, 0).1;

    println!("{answer}");
}

fn merge_sort(nums : &mut [i64], mut inv_count : i64) -> (Vec<i64>, i64) {
    let mut nums : Vec<i64> = nums.to_vec();
    let n : usize = nums.len();

    if n <= 2 {
        if n == 2 && nums[0] > nums[1] {
            nums.swap(0,1);
            inv_count += 1;
        }
        return (nums, inv_count);
    }

    let mid : usize = n/2;
    let (left,right) = nums.split_at_mut(mid);
    let (mut left, left_inv_count) = merge_sort(left,inv_count);
    let (mut right, right_inv_count) = merge_sort(right,inv_count);
    
    inv_count += left_inv_count;
    inv_count += right_inv_count;

    let mut result : Vec<i64> = Vec::with_capacity(n);
    let (mut i, mut j) : (usize, usize) = (0,0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
            inv_count += left.len() as i64 - i as i64;
        }
    }

    let mut left_tail = left.split_off(i);
    let mut right_tail = right.split_off(j);
    
    result.append(&mut left_tail);
    result.append(&mut right_tail);

    (result, inv_count)
}
