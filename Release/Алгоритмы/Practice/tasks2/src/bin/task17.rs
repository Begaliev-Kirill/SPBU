fn main () {
    let arr = [10,11,3,12,15,42,34,56,32,2,34,5,21,45,76,2,3,4,5,6,7];
    let mut answer = 0;
    let mut l : usize = 0;
    let s : i32 = 100;

    let mut pref_sum : Vec<i32> = Vec::with_capacity(arr.len() + 1);
    pref_sum.push(0);
    
    let mut pred_pref : i32 = 0;
    for element in arr.iter() {
        let value = element + pred_pref;
        pred_pref = value;
        pref_sum.push(value);
    }
    println!("{:?}", pref_sum);

    'outer: for r in 0..arr.len() {
        if pref_sum[r+1] - pref_sum[l] > s {
            while l < r {
                l += 1;
                if pref_sum[r+1] - pref_sum[l] <= s {
                    answer += r-l+1;
                    continue 'outer
                }
            }
        }
        if pref_sum[r+1]-pref_sum[l] <= s {
            answer += r-l+1;
        }
    }
    println!("{}", answer);
}
