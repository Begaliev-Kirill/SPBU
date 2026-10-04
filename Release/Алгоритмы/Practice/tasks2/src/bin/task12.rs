fn main() {
    let arr = [1,2,3,4,5,0,1,2,3,4,5];
    let mut answer : Vec<i32> = Vec::with_capacity(arr.len());

    let mut stack : Vec<i32> = Vec::new();
    for element in arr.iter() {
        if stack.len() == 0 || *element <= stack[0] {
            answer.push(-1);
        }
        else {
            for min in stack.iter().rev() {
                if *min < *element {
                    answer.push(*min);
                    break
                }
            }
        }
        loop {
            let n : usize = stack.len();
            if n >= 1 && stack[n-1] >= *element {
                stack.pop();
            } else {
                stack.push(*element);
                break
            }
        }
    }
    println!("{:?}",answer);
}
