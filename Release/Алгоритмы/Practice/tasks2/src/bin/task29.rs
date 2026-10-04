fn main() {
    let arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4];
    let k : usize = 4;
    let mut answer : usize = 0;

    let mut pref_sum : Vec<usize> = Vec::with_capacity(arr.len() + 1);
    pref_sum.push(0);
    
    let mut pred_pref : usize = 0;
    for element in arr.iter() {
        let value = element + pred_pref;
        pred_pref = value;
        pref_sum.push(value % k);
    }
    println!("{:?}", pref_sum);

    let mut memo : Vec<Vec<usize>> = vec![Vec::new();k];
    for i in 0..pref_sum.len() {
        answer += memo[pref_sum[i]].len();
        memo[pref_sum[i]].push(i);
    }
    println!("{}", answer);
}
