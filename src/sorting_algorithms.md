# Sorting algorithms

If there is a more efficient way to search for an element in an array, then there must be a more efficient way to sort the array. That is where sorting algorithms come in.

Sorting is the process of arranging data in a specific order. This order can be ascending or descending. It is defined by a comparison function that compares two elements and returns a value that indicates their relative order.

> Imagine you're a student in a classroom. You're trying to sort the students in the classroom based on their grades. You have a list of students and their grades. You want to arrange the students in ascending order of their grades. 
>
> You start by comparing the first two students. If the first student's grade is higher than the second student's grade, you swap them. Then you compare the next two students. If the second student's grade is higher than the third student's grade, you swap them. This process continues until you reach the end of the list.
>
> You repeat this process until you've sorted the entire list. This is a simple sorting algorithm.

## Bubble sort algorithm

Bubble sort is a simple sorting algorithm that works by repeatedly swapping the adjacent elements if they are in the wrong order. The algorithm continues to iterate until the array is sorted. Since it compares N elements to N - 1, it is not efficient for large arrays.

### Algorithm

1. The algorithm takes the array as input.
2. It compares the first two elements of the array.
3. If the first element is greater than the second, the algorithm swaps them.
4. It compares the second and third elements of the array.
5. If the second element is greater than the third, the algorithm swaps them.
6. This goes on till the end of the array. At this point, the largest element is in its correct position.
7. This process is repeated until the array is sorted. 

Now let's write a function that implements the preceding steps:

```rust,ignore
{{#include ./code/algo/bubble_sort.rs:function}}
```

Here, we created a function that accepts an array and gives out the sorted array.
We used a `for` loop to iterate over the array. The first `for` loop is set to iterate N times, where N is the length of the array. This is because we need displace every element to its sorted position. The second `for` loop iterates over the array N - i times, where i is the instance of the first loop. If the first loop has iterated i times, then i elements are sorted. So, in the second loop, we are only comparing N - i elements.

The `if` condition checks if the current element is greater than the next element. If it is, then we swap them. This is done by using the `swap` function. The `swap` function takes two arguments, the first being the index of the first element and the second being the index of the second element.

At the end of the function, we return the sorted array.