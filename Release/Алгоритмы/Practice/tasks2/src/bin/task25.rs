use std::collections::HashMap;

fn main() {
    let arr = [3, -5, 4];
    let mut map: HashMap<i32, i32> = HashMap::new();
    let s: i32 = -1;

    let mut answer: (i32, i32) = (-1, -1);

    for r in 0..arr.len() {
        let need = s - arr[r];

        if let Some(&l) = map.get(&need) {
            answer = (l, r as i32);
        } else {
            map.insert(arr[r], r as i32);
        }
    }

    println!("{:?}", answer);
}
