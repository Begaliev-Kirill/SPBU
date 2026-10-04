fn main() {
    let arr = [3, 1, 2, 4, 1, 5, 2, 2, 3, 1, 6, 0, 4, 2, 1, 3, 5, 2, 4, 1];
    let n : usize = arr.len();
    let mut l_border : Vec<isize> = vec![-1; n];
    let mut r_border : Vec<usize> = vec![n; n];
    let mut answer : usize = 0;
    
    let mut stack : Vec<usize> = Vec::new();
    for i in 0..n {
       while !stack.is_empty() && arr[stack[stack.len()-1]] > arr[i] {
           stack.pop();
       }

       if !stack.is_empty() {
           l_border[i] = stack[stack.len()-1] as isize;
       }
       stack.push(i);
    }
    
    println!("{:?}", l_border);
    stack = Vec::new();

    for i in (0..n).rev() {
       while !stack.is_empty() && arr[stack[stack.len()-1]] >= arr[i] {
           stack.pop();
       }

       if !stack.is_empty() {
           r_border[i] = stack[stack.len()-1];
       }
       
       stack.push(i);
    }
    println!("{:?}", r_border);

    for k in 0..n {
        answer +=  (k as isize - l_border[k]) as usize * (r_border[k]-k) * arr[k] as usize; 
    }

    println!("{}", answer);
}
