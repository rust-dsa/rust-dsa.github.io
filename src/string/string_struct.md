# String

The `String` type is a heap-allocated, mutable sequence of UTF-8 encoded bytes. What this means is that the `String` type is stored on the heap, it can be modified, and it can store any character that is valid UTF-8 including emojis.

## Creating a String

There are two ways to initialize a `String` type.
The first is by converting a string literal to a `String` type using the `to_string` method. The second is by using the `String::from` method which takes a string slice as an argument.

```rust
{{#include struct.rs:string_init}}
```

If you don't know the content of the string yet, you can use the `String::new` method to create an empty `String` type.

```rust
{{#include struct.rs:string_new}}
```

## Extending a String
There are two ways to insert a string into an existing `String` type. The first is using the `push_str` method which takes a string slice as an argument. The second is using the `push` method which takes a character as an argument.

```rust
{{#include struct.rs:string_push}}
```
