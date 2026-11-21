use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");

    let target : f64 = input
        .trim()
        .parse()
        .expect("not a number");
    
    let l : f64 = 0.0;
    let r : f64 = 10_i32.pow(5) as f64 + 1.0;
    let answer = bin_search(target, l, r);

    println!("{answer}");
}

fn value(x : f64) -> f64 {
    x.powi(2) + x.sqrt()
}

fn abs_diff_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

fn bin_search(target : f64, l: f64, r: f64) -> f64 {
    let mut l : f64 = l;
    let mut r : f64 = r;
    
    let x : f64 = (r + l) / 2.0;
    if abs_diff_eq(value(x), target, 1e-6) {
        return x;
    }

    if value(x) < target {
        l = x;
    } else {
        r = x;
    }

    bin_search(target, l, r)
}
