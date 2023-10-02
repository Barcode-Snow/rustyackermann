fn main() {
    let m: i128 = 3;
    let n: i128 = 3;
    let result = ackermann(m, n);
    println!("Ackermann({}, {}) = {}", m, n, result);
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