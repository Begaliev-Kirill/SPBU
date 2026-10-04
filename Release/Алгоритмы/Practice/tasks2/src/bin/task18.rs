fn main() {
    let arr1 = [1,2,5,7,8,9];
    let arr2 = [1,3,4,5,10,11,12,13,14,15,16];
    let n1 : usize = arr1.len();
    let n2 : usize = arr2.len();
    let mut answer : Vec<i32> = Vec::with_capacity(n1 + n2); 

    let mut i : usize = 0;
    let mut j : usize = 0;
    
    loop {
        if arr1[i] <= arr2[j] {
            answer.push(arr1[i]);
            i += 1;
        } else {
            answer.push(arr2[j]);
            j += 1;
        }

        if i == n1 {
            while j < n2 {
                answer.push(arr2[j]);
                j += 1;
            }
            break
        } else if j == n2 {
            while i < n1 {
                answer.push(arr1[i]);
                i += 1;
            }
            break
        }
    }

    println!("{:?}", answer);
}
