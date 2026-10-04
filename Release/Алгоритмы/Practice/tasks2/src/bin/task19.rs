fn add_on_slice(diff : &mut Vec<i32>, l : usize, r: usize, c : i32) {
    diff[l] += c;
    diff[r+1] -= c;
}

fn main() {
    let arr = [1,7,5,4,12,11,10,5,3,8,24];
    let n : usize = arr.len();
    let mut diff : Vec<i32> = vec![0;n];
    let mut answer : Vec<i32> = vec![0;n];

    add_on_slice(&mut diff, 1, 1, 5);
    add_on_slice(&mut diff, 7, 9, 10);
    add_on_slice(&mut diff, 2, 8, 2);
    diff[0] = arr[0];

    for i in 1..n {
        diff[i] += arr[i] - arr[i-1];
    }
    println!("{:?}", diff);
    
    answer[0] = diff[0];
    for j in 1..n {
        answer[j] = diff[j] + answer[j-1];
    }
    println!("{:?}", answer);
}

