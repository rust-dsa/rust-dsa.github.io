# Bubble sort algorithm

Bubble sort is a simple sorting algorithm that works by repeatedly swapping the adjacent elements if they are in the wrong order. The algorithm continues to iterate until the array is sorted. Since it compares N elements to N - 1, it is not efficient for large arrays.

### Algorithm

1. Take the array as input.
2. Compare the first two elements of the array.
3. If the first element (j) is greater than the next (j + 1), swap them.
4. This goes on till the end of the array. At this point, the largest element is in its correct position.
5. In the second iteration, compare only n - 1 elements, as the last one in its position.
6. This process is repeated until the array is sorted.

Now let's write a function that implements the preceding steps:

```rust,ignore
{{#include ./bubble_sort.rs:function}}
```

Here, we created a function that accepts an array and gives out the sorted array.
We used a `for` loop to iterate over the array. The first `for` loop is set to iterate N times, where N is the length of the array. This is because we need displace every element to its sorted position. The second `for` loop iterates over the array N - i times, where i is the instance of the first loop. If the first loop has iterated i times, then i elements are sorted. So, in the second loop, we are only comparing N - i elements.

The `if` condition checks if the current element is greater than the next element. If it is, then we swap them. This is done by using the `swap` function. The `swap` function takes two arguments, the first being the index of the first element and the second being the index of the second element.

At the end of the function, we return the sorted array.

### Result

```rust
{{#include ./bubble_sort.rs:bubble_sort}}
```

### Time and Space Complexity

The time complexity of bubble sort is O(n^2), where n is the number of elements in the array. This is because the algorithm compares each element with every other element in the array, resulting in quadratic time complexity.

The space complexity of bubble sort is O(1), as it does not require any extra space other than the input array itself.
