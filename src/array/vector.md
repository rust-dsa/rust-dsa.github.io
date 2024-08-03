# Vector type
Unlike arrays, a vector stores the size of the array as well. Arrays don't need to store its size. That is why we can access an array element even when we exceed the actual capacity. The boundaries are not defined.

Another thing about vectors is they're generally stored in a heap and they have a large size allocated than being used. As a result, when new values are inserted, the whole vector does not to be relocated to fulfill the size requirements.

As arrays and vectors, both copy items into another larger array / vector once more element(s) are added, vectors do the copying less often as they already have more capacity than the actual length.

"A vector is a contiguous growable array type with heap allocated contents".[^1]

## Initializing a vector
While initializing a primitive array, we had to specify the size of the array before-hand. There is no such constraints with a vector

```rust
{{#include ./vectors.rs:initialize}}
```

## Accessing and modifying a vector
Similar to the array, we can access individual elements of a vector using its index and modify them if the vector is mutable.
```rust
{{#include ./vectors.rs:access_and_modify}}
```

Now, let's delve into some of the vector specific functions that will be helpful in learning and applying algorithms.

## Vector specific functions

1. `push()`: This function inserts an element at the end of the vector
2. `pop()`: This function removes an element from the end of the vector
3. `len()`: This function outputs the number of elements in the vector

We'll be using these functions extensively. See [docs](https://doc.rust-lang.org/std/vec/struct.Vec.html) for all the functions.

## Advantages and disadvantages of vectors
| **Advantages** | **Disadvantages** |
| --- | --- |
| Dynamic size | Slower access due to indirection |
| Easy insertion and deletion | More memory usage due to extra capacity |
| Efficient use of memory | More complex implementation |
| Flexibility in size | Potential for reallocation |


## 2D vector
As with arrays, vectors are are also two dimensional. The idea of a 2D vector is the same of a [2D array](./primitive.md#2d-array).

**Initializing a 2D array**:
```rust
{{#include ./vectors.rs:2D_vector}}
```

Notice that we have initialized the type inside the fish as another vector. And inside that vector, we have specified the data type.

[^1]: <https://doc.rust-lang.org/std/vec/index.html>