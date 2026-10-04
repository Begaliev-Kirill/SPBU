use std::collections::VecDeque;

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
    let arr = [1, 2, -10, 5, 5, 5, 5, -10, 8, 8, 8, 8, 8, 8, -20, 9, 9, 9, 9, 9];
    let mut deque: VecDeque<usize> = VecDeque::new();
    let k: usize = 3;
    let n: usize = arr.len();
    let mut r: usize = k - 1;

    for i in 0..=r {
        push_deque(&arr, &mut deque, i);
    }
    let mut answer: Vec<i32> = Vec::with_capacity(n - k + 1);
    answer.push(arr[deque[0]]);

    for _ in 0..n - k {
        r += 1;
        push_deque(&arr, &mut deque, r);
        validate_deque(&mut deque, k, r);
        answer.push(arr[deque[0]]);
    }

    println!("{:?}", answer);
}
