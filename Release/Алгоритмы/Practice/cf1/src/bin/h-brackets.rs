use std::io::{self, BufRead, Write};

fn main() {
    let mut input = Vec::new();
    io::stdin()
        .lock()
        .read_until(b'\n', &mut input)
        .expect("input error");

    let mut stack : Vec<u8> = Vec::with_capacity(input.len());
    let mut ok : bool = true;

    for &c in input.iter() {
        match c {
            b'[' => stack.push(b']'),
            b'(' => stack.push(b')'),
            b'{' => stack.push(b'}'),
            b']' | b')' | b'}' => {
                if stack.pop() != Some(c) {
                    ok = false;
                    break;
                }
            },
            _ => ()
        }
    }

    let out : &[u8] = if ok && stack.is_empty() {b"YES\n"} else {b"NO\n"};
    let _ = io::stdout().write_all(out);
}
