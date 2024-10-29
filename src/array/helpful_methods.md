# Problem Solving Preparation

Now that we have learned the theoretical part of arrays and vectors, we'll be moving on to the more practical part, i.e., problem solving. In this section, we'll learn some methods of vectors that will come in handy, especially if you are solving problems in Rust.

> There are several websites on the internet that provide a platform to solve DSA problems, such as:
> * [HackerRank](https://hackerrank.com/)
> * [LeetCode](https://leetcode.com/problemset/)
> * [HackerEarth](https://hackerearth.com/)
> * [CodeWars](https://codewars.com/)
>
> Most of the problems will originate from CodeWars, as it fulfills our requirements as beginners and for our Rust problem-solving setup. There are no constraints on using any of the sites.

In the ['Array or Vector'](./array_or_vector.md) section, we saw the benefits of choosing vectors over arrays. Most of the time, we'll solve problems where the data structure is a vector, but some questions will have an array data structure. We'll see how we can solve problems in both types, but if you want to change the data type from an array to a vector, simply use the `to_vec` method.
```rust,ignore
{{#include ./helpful_methods.rs:to_vec}}
```

## Loops

As we're using vectors as our data structures, it is only natural that we can use loops to iterate over them.

Let's see some different types of loops that we can use to iterate over vectors based on your needs.

### 1. Iterating over the entire vector

The first situation that we'll encounter is when we want to iterate over the entire vector. Here, we can use loops that fundamentally do not break while iterating.

#### 1.1 for loop
The `for` loop is the more versatile loop in Rust. It allows us to iterate over the entire vector (or a portion of it).

> Suppose we have a vector of integers, and we want to complete a function that outputs the sum of all the elements in the vector. Some of the ways we can do this are:

```rust,ignore
{{#include ./helpful_methods.rs:for_loop}}
```
#### 1.2 iter method
Another way to iterate over the entire array is to use the `iter` method. This method returns an (iterator)[https://doc.rust-lang.org/std/slice/struct.Iter.html], which is an object that we can use to iterate over the vector.

The `iter()` is not helpful alone. So we pair it up with different methods depending on our requirements.

1. using closure
```rust,ignore
    {{#include ./helpful_methods.rs:for_each}}
```

2. using inbuilt method
```rust,ignore
{{#include ./helpful_methods.rs:iter_sum}}
```


### 2. Iterating over a portion of the vector

Using `while` loop or `loop` loop, we can iterate over a portion of the vector.





<hr/>

For more helpful vector methods, check out the [Rust documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html#).
