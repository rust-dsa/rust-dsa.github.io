# Recursion

Recursion is a fundamental concept in algorithm design that allows a function to call itself. This technique enables us to break down complex problems into smaller, more manageable sub-problems.

At its core, recursion is a method where the solution to a problem depends on solutions to smaller instances of the same problem. These sub-problems, after being solved individually, provide a sub-solution that can be used to create the solution to the original problem.

There are two key sections to recursion:

1. If the given problem can be solved directly, solve it.
2. Otherwise, divide it into one or more simpler instances of the same problem.

But what if the simpler problem needs to be broken down into an even simpler problem, and again and again, potentially leading to an infinite loop because it cannot be solved directly?

To avoid this infinite loop, a recursive solution must have a base case that can be reached at the end.

Our task is to reduce a problem into a recursive instance and a base case.

**Let's write a recursive function that outputs the factorial of a number n.**

A factorial of a number n is the multiplication of every number from 1 to n (inclusive). It is indicated by an exclamation mark after the number. (n!)

So the factorial of 4 would be: 4 * 3 * 2 * 1 = 24.  <br/>
And the factorial of 5 would be: 5 * 4 * 3 * 2 * 1 = 120.

Huh, I see a pattern here. 🤔 <br/>
If I want the factorial of n, I could just multiply n with the factorial of n - 1. i.e.

`n! = n * (n - 1)!`

6! = 6 * (6 - 1)! <br/>
5! = 5 * (5 - 1)! <br/>
... <br/>
1! = 1
<br/>

I think we have the base case and the recursive case for making our function. The base case would be when n reaches 1, and the recursive case would be multiplying n with the factorial of n - 1.

This is the function

```rust,ignore
{{#include recursion.rs:factorial}}
```

Run the below code for different values of `n`.
```rust,editable
fn main() {
    print!("{} ", factorial(5));
}

fn factorial(n: u32) -> u32 {
    if n == 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}
```

<!-- diagram -->

Now let's write one more recursive problem to solidify our understanding.

## Fibonacci Series

The Fibonacci series is a series of numbers where a number is the addition of the last two numbers, starting with 0 and 1.

0, 1, 1, 2, 3, 5, 8, 13, 21, 34, and so forth.

We need to make a function that outputs the nth number in the Fibonacci series.

Here's what we know:
1. The series starts with 0 and 1.
2. To find the ith number, we need to add the (i - 1)th with the (i - 2)th number.

These are our cases

```rust,editable
fn main() {
    print!("{} ", fibonacci(7));
}

fn fibonacci(n: u32) -> u32 {
    // base case: number is 0 or 1
    if n < 2 {
        return n;
    }
    // recursive case: add previous two numbers to find the current fibonacci number
    fibonacci(n - 1) + fibonacci(n - 2)
}
```

<!-- diagram -->