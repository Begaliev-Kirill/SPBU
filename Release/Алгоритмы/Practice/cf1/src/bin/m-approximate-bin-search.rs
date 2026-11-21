use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");

    let mut iter = input.split_whitespace();
    let count_search: usize = iter.next().unwrap().parse().unwrap();
    let count_targets : usize = iter.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("input 2 error");
    
    let mut nums : Vec<isize> = input
        .split_whitespace()
        .take(count_search)
        .map(|c| c.parse().expect("not a number"))
        .collect();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("input 2 error");
    
    let targets : Vec<isize> = input
        .split_whitespace()
        .take(count_targets)
        .map(|c| c.parse().expect("not a number"))
        .collect();
    
    for i in 0..count_targets {
        let l : usize = 0;
        let r : usize = count_search - 1;
        
        if nums[r] <= targets[i] {
            println!("{}", nums[r]);
            continue;
        } else if nums[l] >= targets[i] {
            println!("{}", nums[l]);
            continue;
        }
        
        let answer : isize = bin_search(&mut nums,targets[i],l,r);
        println!("{answer}");
    }
}

fn bin_search(nums : &mut Vec<isize>, target : isize, l: usize, r: usize) -> isize {
    let l : usize = l;
    let r : usize = r;
    
    if nums[l] == target {
        return nums[l];
    } else if nums[r] == target {
        return nums[r];
    } else if l == r {
        return nums[l];
    } else if r - l == 1 {
        if (target - nums[l]).abs() <= (target-nums[r]).abs() {
            return nums[l];
        } else {
            return nums[r];
        }
    }
    
    let mid = (l + r) / 2;

    if nums[mid] < target {
        return bin_search(nums, target, mid, r);    
    }

    bin_search(nums, target, l, mid)
}
