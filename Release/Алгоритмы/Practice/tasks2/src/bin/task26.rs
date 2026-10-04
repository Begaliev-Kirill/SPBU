use std::collections::VecDeque;
use std::cmp::max;

fn push_min_stack(arr : &[usize], min_stack: &mut VecDeque<usize>, element : usize) {
    while !min_stack.is_empty() && arr[min_stack[min_stack.len()-1]] >= arr[element] {
        min_stack.pop_back();
    }
    min_stack.push_back(element);
}

fn push_max_stack(arr : &[usize], max_stack: &mut VecDeque<usize>, element : usize) {
    while !max_stack.is_empty() && arr[max_stack[max_stack.len()-1]] <= arr[element] {
        max_stack.pop_back();
    }
    max_stack.push_back(element);
}

fn main() {
    let arr = [4, 1, 7, 3, 9, 2, 8, 5, 6, 10, 15, 12, 18, 11, 14, 20, 13, 19, 16, 17];
    let k : usize = 6;
    let mut answer : usize = 0;

    let mut min_stack : VecDeque<usize> = VecDeque::new();
    let mut max_stack : VecDeque<usize> = VecDeque::new();
    let mut l : usize = 0;

    for r in 0..arr.len() {
        push_min_stack(&arr, &mut min_stack, r);
        push_max_stack(&arr, &mut max_stack, r);
        while !min_stack.is_empty() && 
              !max_stack.is_empty() && 
              arr[max_stack[0]] - arr[min_stack[0]] > k 
        {
            l += 1;
            while min_stack[0] < l {
                min_stack.pop_front();
            }
            while max_stack[0] < l {
                max_stack.pop_front();
            }
        }
        answer = max(answer,r-l+1);
    }

    println!("{}", answer);
}
