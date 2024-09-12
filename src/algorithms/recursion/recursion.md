# Recursion

Recursion is a fundamental concept used in designing algorithms. It allows a function to call itself. This technique allows us to break down complex problems into smaller, more manageable sub-problems.

At its core, recursion is a method in which the solution to a problem depends on solutions to smaller instances of the same problem. These sub-problems, after being solved individually provide a sub-solution that can be used to create the solution to the original problem.

There are two sections to recursion

1. If the given problem can be solved directly, solve it.
2. Else divide it to one or more simpler instances of the same problem.

But what if the simpler problem has to be broken down into a simpler-ler problem. And again again for infinity because it cannot be solved directly?

To avoid the infinity loop, a recursive solution has to have a base case that can be reached at the end.

Our only task is to reduce a problem into a recursive instance and a base case.

**Let's write a recursive function that outputs the factorial of a number n.**

A factorial of a number n is the multiplication of every number from 1 to n (inclusive). It is indicated by exclamation mark after the number. (n!)

So the factorial of 4 would be: 4 * 3 * 2 * 1 = 24.  <br/>
And the factorial of 5 would be: 5 * 4 * 3 * 2 * 1 = 120.

Huh, I see a pattern here. 🤔 <br/>
If I want the factorial of n, I could just multiply n with factorial of n - 1. i.e.

`n! = n * (n - 1)!`

6! = 6 * (6 - 1)! <br/>
5! = 5 * (5 - 1)! <br/>
... <br/>
1! = 1
<br/>

I think we have the base case and the recursive case for making our function. The base case would be when n reaches 1, and the recursive case would be multiplying n with factorial of n - 1.

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
        return 1
    } else {
        return n * factorial(n - 1);
    }
}
```