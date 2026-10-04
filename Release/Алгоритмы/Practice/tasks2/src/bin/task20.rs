fn main() {
    let mut arr : Vec<i32> = vec![0,2,1,1,1,2,0,0,0,1,1,2,2,2,0,1,2,2,1,0,1,0,2];
    let n : usize = arr.len();

    let mut zero_ptr = 0;
    let mut two_ptr = n-1;

    while arr[zero_ptr] == 0 {
        zero_ptr += 1;
    }
    while arr[two_ptr] == 2 {
        two_ptr -= 1;
    }

    let mut one_ptr = zero_ptr;

    loop {
        match arr[one_ptr] {
            1 => one_ptr += 1,
            0 => {
                arr[one_ptr] = arr[zero_ptr];
                arr[zero_ptr] = 0;
                one_ptr += 1;
                zero_ptr += 1;
            },
            2 => {
                arr[one_ptr] = arr[two_ptr];
                arr[two_ptr] = 2;
                two_ptr -= 1;
            }
            _ => break
        }

        if one_ptr > two_ptr {
            break
        }
    }

    println!("{:?}", arr);
}
