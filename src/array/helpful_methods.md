# Preparing for Problem Solving

Now that we have learned the theoretical part of arrays and vectors, we'll be moving on to the more practical part, i.e., problem solving. In this section, we'll learn some methods of vectors that will come in handy, especially if you are solving problems in Rust.

> There are several websites on the internet that provide a platform to solve DSA problems, such as:
> * [HackerRank](https://hackerrank.com/)
> * [LeetCode](https://leetcode.com/problemset/)
> * [HackerEarth](https://hackerearth.com/)
> * [CodeWars](https://codewars.com/)
>
> Most of the problems will originate from CodeWars, as it fulfills our requirements as a beginner and for our Rust problem-solving setup. There is no constraint on using any of the sites.

In the ['Array or Vector'](./array_or_vector.md) section, we saw the benefits of choosing vectors over arrays. Most of the time, we'll solve problems where the data structure is a vector, but some questions will have an array data structure. We'll see how we can solve problems in both types, but if you want to change the data type from an array to a vector, simply use the `to_vec` method.

```rust,ignore
{{#include ./helpful_methods.rs:to_vec}}
