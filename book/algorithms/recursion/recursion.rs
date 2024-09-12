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