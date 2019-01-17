pub fn find() -> Option<u32> {
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    let mut c: u32 = 0;
    for m in 1..500 {
        let n: u32 = m as u32 + 1;
        a = n.pow(2) - (m as u32).pow(2);
        b = 2 * n * m;
        c = n.pow(2) + m.pow(2);
        if a + b + c == 1000 {
            println!("FOUND CANDIDATES: a: {}, b: {}, c: {}", a, b, c);
        }
    }
    Some(a + b + c)
}
