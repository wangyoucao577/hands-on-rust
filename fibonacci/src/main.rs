
fn main() {
    for i in 0..11 {
        println!("fibonacci({}) = {}", i, fibonacci(i));
    }
}

// given 'n', return value of index 'n'(start from 0)
// for example, fibonacci(5) = 5
// 0, 1, 1, 2, 3, 5, 8, 13, 21, ...
fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n
    }

    let mut v1 = 0;
    let mut v2 = 1;
    for _ in 1..n {
        let tmp = v2;
        v2 = v2 + v1;
        v1 = tmp;
    }

    v2
}
