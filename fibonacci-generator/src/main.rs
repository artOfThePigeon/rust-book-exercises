fn main() {
    let iterations: u32 = 7;
    fibonacci(iterations);
}
fn fibonacci(n: u32) {
    let mut fib: u32 = 1;
    let mut fib_previous = 0;
    for _ in 1..=n {
        println!("{fib}");
        fib = fib + fib_previous;
        fib_previous = fib - fib_previous;
    }
}
