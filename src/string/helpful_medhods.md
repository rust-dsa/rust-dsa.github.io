# Problem Solving Preparation

Let's look at some helpful methods that will help you in solving coding problems on strings.

### Convert String to &str
```rust,ignore
{{#include ./helpful_methods.rs:as_str}}
```

### Convert &str to String
```rust,ignore
{{#include ./helpful_methods.rs:to_string}}
```

### Create a String with given values
```rust
{{#include ./helpful_methods.rs:format}}
```

### Replace a slice or a character in a string
```rust
{{#include ./helpful_methods.rs:replace}}
```
### Convert a &str into Vec<char>
If you want to access individual values in a string slice, it is better to convert it into a vector of chars
```rust,ignore
{{#include ./helpful_methods.rs:to_vector}}
```
The `chars` method returns an iterator over the individual characters of the slice and the `collect` method takes the characters and turn them into a collection.

### Access nth character in a string
The `nth()` is used to return the nth element of the iterator. This means `nth` must be used with an iterator or in case of a string, `chars()`.

```rust
{{#include ./helpful_methods.rs:nth}}
```

### Split a string
The `split`[^2] method in Rust is used to divide a string slice into an iterator of substrings based on a specified delimiter.

```rust
{{#include ./helpful_methods.rs:split}}
```

[^2]: https://doc.rust-lang.org/std/primitive.str.html#method.split
