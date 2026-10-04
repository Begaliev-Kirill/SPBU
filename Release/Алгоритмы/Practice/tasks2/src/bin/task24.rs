fn main() {
    let arr = [0,1,2,0,3,0,1,2,0,0,4,2,1,2,5,0,1,2,0,2];

    let mut l = 0;
    let mut r = arr.len() - 1;
    let mut l_max = 0;
    let mut r_max = 0;
    let mut answer = 0;

    while l < r {
        if arr[l] <= arr[r] {
            if arr[l] >= l_max {
                l_max = arr[l];
            } else {
                answer += l_max - arr[l];
            }
            l += 1;
        } else {
            if arr[r] >= r_max {
                r_max = arr[r];
            } else {
                answer += r_max - arr[r];
            }
            r -= 1;
        }
    }

    println!("{}", answer);
}
