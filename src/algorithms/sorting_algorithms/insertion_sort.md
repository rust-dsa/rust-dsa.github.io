# Insertion sort algorithm


Insertion sort is another simple sorting algorithm that works by repeatedly inserting an element from the unsorted part of the array into its correct position in the sorted part of the array. The algorithm continues to iterate until the array is sorted.

### Algorithm

1. Take the array as input.
2. Iterate over the array.
3. Compares the current element with the previous element.
4. If the current element is smaller than the previous element, swap them.
5. This goes on till the end of the array.
6. This process is repeated until the array is sorted.

Now let's write a function that implements the preceding steps:

```rust,ignore
{{#include ./insertion_sort.rs:function}}
```

We use a `for` loop to iterate over the array. Inside the loop, we initialize a variable `j` to the current index of the array. We then use a `while` loop to iterate over the array till j is greater than 0 and the current element is smaller than the previous element. If both conditions are true, then we swap current element with the previous element. We then decrement j by 1 to move to the previous element and check if the condition is true again.

At the end of the function, we return the sorted array.

### Result

```rust
{{#include ./insertion_sort.rs:insertion_sort}}
```

### Time and Space Complexity

The time complexity of insertion sort is O(n^2), where n is the number of elements in the array. This is because the algorithm compares each element with every other element in the array, resulting in quadratic time complexity.

The space complexity of insertion sort is O(1), as it does not require any extra space other than the input array itself.