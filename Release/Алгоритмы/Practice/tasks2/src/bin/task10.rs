fn main() {
    let arr = [1,2,3,4,5,6,20,34,254];
    let target : i32 = 60;
    let n: usize = arr.len();

    let mut pref_sum : Vec<i32> = Vec::with_capacity(arr.len() + 1);
    pref_sum.push(0);
    
    let mut pred_pref : i32 = 0;
    for element in arr.iter() {
        let value = element + pred_pref;
        pred_pref = value;
        pref_sum.push(value);
    }
    println!("{:?}", pref_sum);

    let mut l : usize = 0;
    let mut r : usize = 1;

    let mut answer : (usize,usize) = (0,0);
    loop {
        match pref_sum[r] - pref_sum[l] {
            x if l == r => {if x/2 == target {answer = (l,r)};break},
            x if x < target => r += 1,
            x if x > target => l += 1,
            x if x == target => {answer = (l,r); break}
            _ => break
        }
        if r == n {
            break
        }
    }
    println!("{:?}", answer);
}
