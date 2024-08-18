fn main() {
    println!("{}", fibonacci(190));
}

fn fibonacci(size: u128) -> u128 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..size {
        let tmp = a;
        a = b;
        b = a + tmp;
    }

    b
}