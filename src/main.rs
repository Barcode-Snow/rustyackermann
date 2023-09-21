fn main() {
    let m:i128= 5;
    let n:i128 = 5;
    ackermann(m, n);
}

fn ackermann(mut m:i128, mut n:i128){
    if m == 0 {
        n = n +1;
        println!("n:{}",n);
    }
    else if n == 0 {
        m=m-1;
        ackermann(m, n);
    }
    else {
        n = n-1;
        ackermann(m, n);
        m=m-1;
        ackermann(m, n);
    }
}


