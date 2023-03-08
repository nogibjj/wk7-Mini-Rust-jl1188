pub fn hamming_weight(n: u32) -> i32 {
    let mut res = 0;
    let mut n = n;
    while n != 0 {
        res += 1;
        let small = n-1;
        n &= small;
    }
    res
}
