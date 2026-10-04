use std::cmp::Ordering;

fn main() {
    let arr1 = [1,2,4,5,7,10,11];
    let arr2 = [2,3,4,5,8,9,12];
    let n1 = arr1.len();
    let n2 = arr2.len();
    let mut answer : Vec<i32> = Vec::new();

    let mut i1 : usize = 0;
    let mut i2 : usize = 0;

    loop {
        match arr1[i1].cmp(&arr2[i2]) {
            Ordering::Greater => i2 += 1,
            Ordering::Equal => {answer.push(arr1[i1]);i1+=1;i2+=1;},
            Ordering::Less => i1 += 1,
        }
        if i1 == n1 || i2 == n2 {
            break
        }
    }

    println!("{:?}", answer);
}
