fn ackermann(m: i128, n: i128) -> i128 {
    let mut stack = Vec::new();
    let mut m = m;
    let mut n = n;
    
    loop {
        if m == 0 {
            if let Some((prev_m, n)) = stack.pop() {
                m = prev_m - 1;
                stack.push((m, n + 1));
            } else {
                return n + 1;
            }
        } else if n == 0 {
            m -= 1;
            n = 1;
        } else {
            stack.push((m - 1, n));
            n -= 1;
        }
    }
}

fn main() {
    for i in 1..=100 {
        for j in 1..=100 {
            let result = ackermann(i, j);
            println!("Ackermann({}, {}) = {}", i, j, result);
        }
    }
}
