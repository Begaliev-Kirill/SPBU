use std::cmp::max;

fn main() {
    let arr = [-5,3,4,2,10,12,-245,3,34,5,56,-50,75,0,-50,35];
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

    let mut answer : i32 = 0;
    loop {
        answer = max(answer,pref_sum[r] - pref_sum[l]);
        if pref_sum[r] < pref_sum[l] {
            l = r;
        }
        r += 1;
        if r == n {
            break
        }
    }
    println!("{:?}", answer);
}
