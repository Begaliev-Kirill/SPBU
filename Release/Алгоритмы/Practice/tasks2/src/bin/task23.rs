use std::collections::VecDeque;
use std::cmp::max;

fn push_deque(arr: &[i32], deque: &mut VecDeque<usize>, i: usize) {
    while let Some(&back) = deque.back() {
        if arr[back] >= arr[i] {
            deque.pop_back();
        } else {
            break;
        }
    }
    deque.push_back(i);
}

fn validate_deque(deque: &mut VecDeque<usize>, k: usize, i: usize) {
    loop {
        if let Some(&front) = deque.front() {
            if front + k <= i {
                deque.pop_front();
            } else {
                break;
            }
        } else {
            break;
        }
    }
}
fn main() {
    //let arr = [1, 2, -10, 5, 5, 5, 5, -10, 8, 8, 8, 8, 8, 8, -20, 9, 9, 9, 9, 9];
    let arr =[10, 20, 20, 20, -50, 20, 20, 20, 20, 20, -10, 20, 20, 20, 20, 5, 5, 5, 5, 5];

    let mut deque: VecDeque<usize> = VecDeque::new();
    let l_win : usize = 3;
    let r_win : usize = 5;
    let k: usize = r_win-l_win+1;
    let n: usize = arr.len();
    let mut answer : i32 = 0; 
    let mut pref_sum : Vec<i32> = Vec::with_capacity(arr.len() + 1);
    
    let mut pred_pref : i32 = 0;
    for element in arr.iter() {
        let value = element + pred_pref;
        pred_pref = value;
        pref_sum.push(value);
    }
    println!("{:?}", pref_sum);

    for i in 0..=k-1 {
        push_deque(&pref_sum, &mut deque, i);
    }

    let mut min_prefs: Vec<i32> = Vec::with_capacity(n - k + 1);
    min_prefs.push(pref_sum[deque[0]]);

    for r in k..n{
        push_deque(&pref_sum, &mut deque, r);
        validate_deque(&mut deque, k, r);
        min_prefs.push(pref_sum[deque[0]]);
    }
    println!("{:?}", min_prefs);
    
    for j in r_win..pref_sum.len() {
        answer = max(answer, pref_sum[j]-min_prefs[j-k-2]);
    }

    println!("{}", answer);
}
