# String

A string is a data structure that holds a sequence of character. Strings very similar to arrays. On an application level, let's compare strings to arrays.

| Array | String |
|---|---|
| Contains similar data items in contiguous locations | Contains characters in contiguous locations |
| Elements can be accessed using an index | Characters can be accessed using an index (e.g., `chars()` or indexing with `[]` in `&str`) |
| Size can be determined using `len()` | Length can be determined using `len()` |
| Elements can be iterated over using `iter()` | Characters can be iterated over using `chars()` or `bytes()` |
| Elements can be modified (e.g., using `push()` or indexing with `[]`) | Characters can be modified (e.g., using `push_str()` or `replace_range()`) |

<br/><br/>
Strings are a fundamental data structure of any programming language. In Rust, there are three types of strings:

1. The string slice: `str`
2. The string struct: `String`