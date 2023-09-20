fn main() {
    let mut m:i128= 0;
    let mut n:i128 = 0;
    ackermann(m, n);
}

fn ackermann(m:i128, n:i128){
    if m == 0 {
        n = n +1;
    }
    else if n == 0 {
        m=m-1;
        ackermann(m, n);
    }
    else {
        n = n-1;
        n = ackermann(m, n);
        m=m-1;
        ackermann(m, n);
    }
}