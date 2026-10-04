fn main() {
    let mut arr = [1,15,23,34,64,32,45,12,3,54,33,78,87,65,77,56,25,54];
    arr.sort();
    let w : i32 = 100;

    let mut l : usize = 0;
    let mut r : usize = arr.len()-1;
    let mut answer : i32 = 0;
    while l <= r {
        if l == r {
            answer += 1;
            break;
        }

        if arr[l] + arr[r] <= w {
            answer += 1;
            l += 1;
            r -= 1;
        } else {
            answer += 1;
            r -= 1;
        }
    }

    println!("{}",answer);
}
