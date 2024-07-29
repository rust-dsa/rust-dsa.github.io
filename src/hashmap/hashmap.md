# Hash map

A hash map is a data structure that efficiently stores key-value pairs. Think of it as a dictionary where you can look up words (keys) that point to its meaning (value).

<!-- TODO add hashmap image -->

To use Hash map in your Rust code, you'll need to import it from `collections`. You can do that by adding the following line to your code:

```rust,ignore
{{#include hashmap.rs:imports}}
```

## Creating a HashMap

To create a hash map, you'll need to use the `HashMap::new()` method. This method returns an empty hash map. You can also use the `HashMap::with_capacity()` method to create a hash map with a specific capacity. The capacity is the number of elements the hash map can hold without resizing.

```rust,ignore
{{#include hashmap.rs:new}}
```

## Inserting a key-value pair

To insert a key-value pair into a hash map, you'll need to use the `insert()` method. This method takes two arguments: the key and the value.

```rust,ignore
{{#include hashmap.rs:insert}}
```

## Accessing a value

To access a value in a hash map, you'll need to use the `get()` method. This method returns an `Option` type, so you'll need to unwrap the value to access it.

```rust,ignore
{{#include hashmap.rs:get}}
```

## Removing a key-value pair

To remove a key-value pair from a hash map, you'll need to use the `remove()` method. This method takes the key as an argument and returns the value associated with that key.

```rust,ignore
{{#include hashmap.rs:remove}}
```

The whole code looks something like this:

```rust
{{#include hashmap.rs:hashmap}}