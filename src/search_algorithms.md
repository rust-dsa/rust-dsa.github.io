# Search algorithms

An important and recurring problem in computing is locating information. More succinctly, this problem is called as searching. To solve this problem, search algorithms are used.

As you may have guessed from the name of the algorithms, search algorithms help us locate data in data structures.


In order to simplify matters, we'll search through items in a array. Recalling what we learned in chapter 2, arrays are a collection of similar items stored in a contiguous location. Searching through it will be simple enough.

The most straightforward approach to searching is to examine each element of the data structure sequentially. Concurrently this searching algorithm is called linear search or sequential search.


## Linear search algorithm
Let's discuss what the search algorithm will take as input and will give out as output.

* Input: 
   1. the array
   2. element to be searched

* Output:
   1. `Some(index)` if element is present in the array
   2. `None` if element is not present in the array

Now that we know what the algorithm expects and what we expect of it, let's understand how it will work

1. The algorithm takes the array and the token (number to be searched) as input
2. It compares the token with every element in the array
3. If the token and element match, the algorithm returns the index of that element
4. If the token doesn't match with any element i.e. it reaches the end of array, return `None`

This is how a linear search algorithm works

Now let's write a function that does the preceding:

```rust,ignore
{{#include ./code/algo/search_algorithms.rs:search_function}}
```

Here, we created a function that accepts an array and the element to be searched and gives out the Option enum.

Inside the function, a `for` loop iterates through every element in the array, that element is compared with given token for equality. If equal, the index of the element enclosed inside `Some` is returned.  When the loop iterates through the array and cannot find the token, then the function returns `None`.

Now that we have a function let's give it the inputs
```rust,ignore
{{#include ./code/algo/search_algorithms.rs:input}}
```
Note that the by default type of both these variables is `Vec<i32>` and `i32` respectively.

By setting up the variables and the outputs in the main function

```rust
{{#include ./code/algo/search_algorithms.rs:main}}
```

## Result

```rust,editable
{{#include ./code/algo/search_algorithms.rs:linear_search}}
```