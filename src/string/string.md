# String

A string is a data structure that holds a sequence of characters. Strings are very similar to arrays. On an application level, let's compare strings to arrays.

| Array | String |
|---|---|
| Contains similar data items in contiguous locations | Contains characters in contiguous locations |
| Elements can be accessed using an index | Characters can be accessed using an index (e.g., `chars()` or indexing with `[]` in `&str`) |
| Size can be determined using `len()` | Length can be determined using `len()` |
| Elements can be iterated over using `iter()` | Characters can be iterated over using `chars()` or `bytes()` |
| Elements can be modified (e.g., using `push()` or indexing with `[]`) | Characters can be modified (e.g., using `push_str()` or `replace_range()`) |

<br/><br/>
Strings are a fundamental data structure in any programming language. In Rust, there are two types of strings:

1. The string slice: `str`, which usually comes in its borrowed form `&str`
2. The string struct: `String`

### Why does Rust have two string types?

On the surface level, string types are pretty simple. You write something in quotation marks, you assign it to a variable, and Voila! , you have a string type variable.

```rust
let init = "Knock Knock";
```

But there is a lot going on behind the scenes, when we use a string. Luckily, Rust provides us two level of controls through its string types. These types are necessary to continue Rust's priority on safety and performance.

We'll be delving deeper into the two types of strings in rust in later sections.
