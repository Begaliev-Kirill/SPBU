fn main() {
    let arr = [1,3,6,10,123,245,254];
    let mut l : usize = 0;
    let mut r : usize = arr.len()-1;
    let target : i32 = 124;
    
    let mut answer : (usize,usize) = (0,0);
    loop {
        match arr[l] + arr[r] {
            x if x > target => r -= 1,
            x if x < target => l += 1,
            x if x == target => {answer = (l,r);break},
            _ => break,
        }
        if l == r {
            break
        }
    }

    println!("{:?}", answer); //(0,0) if no sum in arr
}
