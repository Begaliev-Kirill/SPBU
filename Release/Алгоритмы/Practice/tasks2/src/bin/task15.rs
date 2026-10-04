use std::cmp;

fn main() {
    let arr = [5,7,2,3,2,1,10,11,4,3,2,5,6,2];
    let mut l : usize = 0;
    let mut r : usize = arr.len() - 1;
    let mut answer : i32 = 0;

    loop {
        answer = cmp::max(answer, cmp::min(arr[l], arr[r])*(r-l) as i32);
        if arr[r] < arr[l] {
            r -= 1;
        } else {
            l += 1;
        }
        if r == l {
            break
        }
    }

    println!("{}", answer);
}
