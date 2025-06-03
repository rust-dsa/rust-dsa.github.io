# Selection sort algorithm

Selection sort is another simple sorting algorithm that works by repeatedly finding the minimum/maximum element in the array and swapping it with the current element. The algorithm continues to iterate until the array is sorted.

### Algorithm

1. Take the array as input.
2. Initialize the first element as the current minimum element in the array.
3. Compare the current minimum with each element in the array.
4. If the current element is smaller than the current minimum, swap them.
5. The current minimum (at the end of the iteration) is swapped with element at the beginning of the array.
6. This goes on till the end of the array.

Now let's write a function that implements the preceding steps:

```rust,ignore
{{#include ./selection_sort.rs:function}}
```

Here, we created a function that accepts an array and gives out the sorted array.
We used a `for` loop to iterate over the array. The first `for` loop is set to iterate N - 1 times, where N is the length of the array. This is because we need to swap every element to its sorted position. The second `for` loop starts iterating from i + 1 till the end of the array because the elements before i are already sorted. So, in the second loop, we are only comparing N - i elements.

The `if` condition checks if the current element is smaller than the current minimum. If it is, then we swap them. This is done by using the `swap` function.

At the end of the function, we return the sorted array.

### Result

```rust
{{#include ./selection_sort.rs:selection_sort}}
```

### Time and Space Complexity

The time complexity of selection sort is O(n^2), where n is the number of elements in the array. This is because the algorithm compares each element with every other element in the array, resulting in quadratic time complexity.

The space complexity of selection sort is O(1), as it does not require any extra space other than the input array itself.