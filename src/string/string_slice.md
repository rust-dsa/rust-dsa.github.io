# &str

`&str` is a primitive string type in Rust. Another name for type is 'string slice'. It is called a string slice because it *is* a slice of a string.

Slices are references to the original string, so they don't own the data. This prevents unnecessary copying of data.

Every type of string in Rust is utf-8 encoded. That means we can initialize a string in any languages. We can also include emoticons. Now let's see how it's done.

```rust
{{#include ./slice.rs:slice_example}}
```

When you extract a portion of a string which is `String` or a `&str` type, you create a `&str` type. The extraction is done with specifying the start and end index of the string in square brackets.

```rust
{{#include ./slice.rs:slice_from_string}}
```

<!-- IF you have the knowledge, add more information here -->