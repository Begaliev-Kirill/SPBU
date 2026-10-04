fn main() {
    let arr = [0,1,1,1,0,0,0,1,1,1,1,1,0,1,1,1,0,0,1,0,1,0,1,0];
    let n: usize = arr.len();
    let k : i32 = 4;

    let mut l : usize = 0;
    let mut r : usize = 0;
    let mut zero_count : i32 = 0;
    if arr[l] == 0 {
        zero_count += 1;
    }

    let mut answer : (usize,usize) = (0,0);
    loop {
        r += 1;
        if r == n {
            break
        }

        if arr[r] == 0 {
            zero_count += 1;
        }

        if zero_count > k {
            l += 1;
            while arr[l-1] != 0 {
                l += 1;
            }
            zero_count -= 1;
        }
        
        if r - l + 1 > answer.1 - answer.0 + 1 {
            answer = (l,r);
        }
    }
    println!("{:?}", answer);
}
