use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");
    let mut iter = input.split_whitespace();

    let n : usize = iter.next().unwrap().parse().unwrap();
    let k : usize = iter.next().unwrap().parse().unwrap();

    let mut v : Vec<i64> = vec![0;n];
    let mut w : Vec<i64> = vec![0;n];

    for i in 0..n {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("input 2 error");

        iter = input.split_whitespace();

        let thing_v : i64 = iter.next().unwrap().parse().unwrap();
        let thing_w : i64 = iter.next().unwrap().parse().unwrap();

        v[i] = thing_v;
        w[i] = thing_w;
    }

    //we are tryng to maximize p/q with Dinkelbachs
    let mut p : i64 = 0;
    let mut q : i64 = 1;

    // di = p/q, d = (di, index)
    let mut d : Vec<(i64, usize)> = vec![(0,0);n];
    let mut k_best : Vec<usize> = Vec::with_capacity(k);

    loop {
        for i in 0..n {
            //max(p/q) => max(v-(p/q)*w) => max((v*q)/q-(p/q)*w) => max(v*q-p*q)
            d[i] = (v[i]*q - p*w[i], i);
        }

        d.select_nth_unstable_by_key(n-k, |x| x.0);

        let mut new_p : i64 = 0;
        let mut new_q : i64 = 0;
        
        k_best.clear();
        for j in (n-k)..n {
            let index : usize = d[j].1;
            k_best.push(index+1);
            new_p += v[index];
            new_q += w[index];
        }

        if new_p*q == new_q*p {
            break
        }

        p = new_p;
        q = new_q;
    }

    let mut answer : String = String::with_capacity(k*8);
    for idx in k_best {
        answer.push_str(&format!("{idx}\n").to_string());
    }
    println!("{answer}");
}
