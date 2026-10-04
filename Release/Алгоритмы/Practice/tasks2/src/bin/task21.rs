fn main() {
    let arr = [4,18,2,3,15,27,28,4,25,23,4,3,2,1,1,50,49,38,37];
    let mut stack : Vec<usize> = Vec::new();
    let mut answer : Vec<i32> = vec![0;arr.len()];

    for r in 0..arr.len() {
        while stack.len() >= 1 && arr[stack[stack.len()-1]] < arr[r] {
            answer[stack[stack.len()-1]] = arr[r];
            stack.pop();
        }
        stack.push(r);
    }

    for element in stack {
        answer[element] = -1;
    }

    println!("{:?}", answer);
}
