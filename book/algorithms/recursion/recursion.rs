// ANCHOR: factorial
fn factorial(n: u32) -> u32 {
    if n == 1 {                       // base case
        return 1;
    }
    else {
        return n * factorial(n - 1);  // recursive case
    }
}
// ANCHOR_END: factorial

// ANCHOR: fibonacci
fn fibonacci(n: u32) -> u32 {
    // base case: number is 0 or 1
    if n < 2 {
        return n;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

fn peasant_multiplication(mut x: u8, mut y: u8) -> u8 {
    let mut prod = 0;

    while x > 0 {
        if x % 2 == 1 {
            prod += y;
        }
        x /= 2;
        y += y;
    }
    prod
}