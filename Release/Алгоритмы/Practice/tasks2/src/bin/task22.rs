use std::cmp::max;

fn main() {
    let arr = [6, 2, 5, 4, 5, 1, 6, 3, 3, 2, 5, 4, 2, 1, 3, 4, 5, 6, 2, 4,0];
    let mut answer : usize = 0;
    let mut stack : Vec<usize> = Vec::new();

    for r in 0..arr.len() {
        while !stack.is_empty() && arr[stack[stack.len()-1]] > arr[r] {
            if stack.len() > 1 {
                answer = max(answer, (r-1-stack[stack.len()-2])*arr[stack[stack.len()-1]]);
            } else {
                answer = max(answer, arr[stack[stack.len()-1]]*r);
            }
            stack.pop();
        }
        stack.push(r);
    }

    println!("{}", answer);
}
