fn main() {
    println!("{}", fibonacci(45));
}

fn fibonacci(size: u128) -> u128 {
    if size < 2 {
        return 1;
    }
    return fibonacci(size-1) + fibonacci(size-2);
}