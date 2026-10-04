use std::cmp::Ordering;

fn main() {
    let mut arr : Vec<usize> = vec![4, 1, 7, 3, 9, 2, 8, 5, 6, 10, 15, 12, 18, 11, 14, 20, 13, 19, 16, 17];
    let s : usize = 57;
    let mut answer : bool = false;
    
    arr.sort();
    'outer: for l in 0..arr.len()-2 {
        let mut l1 = l+1;
        let mut r = arr.len()-1;
        let target : usize = s - arr[l];
        loop {
            match (arr[l1] + arr[r]).cmp(&target) {
                Ordering::Less => l1 += 1,
                Ordering::Greater => r -= 1,
                Ordering::Equal => {answer = true; break 'outer;},
            }

            if l1 == r {
                break
            }   
        }
    }

    if answer {
        println!("Yes");
    } else {
        println!("No");
    }
}
