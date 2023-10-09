use std::thread;

fn main() {
    thread::Builder::new().stack_size(32*1024*1024).spawn(move || {
        for i  in 1..=100 {
            let m: i128 = i;
            let n: i128 = i;
            let result = ackermann(m, n);
            println!("Ackermann({}, {}) = {}", m, n, result);
        }
    }).unwrap();
}

fn ackermann(m: i128, n: i128) -> i128 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ackermann(m - 1, 1)
    } else {
        ackermann(m - 1, ackermann(m, n - 1))
    }
}