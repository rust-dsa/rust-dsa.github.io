## Binary search algorithm
When we write an algorithm, we have to consider the possibility that it can be improved in terms of performance. Performance may be improved as a result of less time required, and/or less space occupied.  Binary search algorithm is the improvement of linear search in that it reduces the time required to search for an element.

You may ask yourself, "Why not stick with linear search? It gets the work done." While we're working with minimal data in our examples, real-world scenarios often involve dealing with billions of units of data. In such cases, linear search would take O(5,000,000,000) on average and O(10,000,000,000) at worst. Those numbers don't look good, do they? That's why we need a more efficient algorithm for large datasets.

> You recently stumbled upon a word - Facet: a distinct feature or element in a problem - that you don't know the meaning of. And as you are a sane person, chose to find your answer in a dictionary.
>
> You start by opening the dictionary right in the middle section. You see words starting with 'M'. Now you know that 'F', the first letter in you word, comes before 'M'. So you discard the letters after 'M'. Now the word you are looking for is somewhere between 'A' and 'M'.
>
> You open the section roughly between 'A' and 'M', your new, smaller dictionary. You have found the 'F' section of the dictionary. Now you can choose to linearly search through the words.
>
> With every iteration, you cut the search area in half. The first time you were searching through 26 letters. The second, you were searching through only 13. The next time would be half of that, and so on.

Binary search is an efficient algorithm for finding a specific element in a sorted array. It works by repeatedly dividing the search interval in half.

### Algorithm

1. The algorithm starts with the entire sorted array.
2. It compares the target value with the middle element of the array.
3. If the target matches the middle element, the search is complete.
4. If the target is less than the middle element, the search continues in the lower half of the array.
5. If the target is greater than the middle element, the search continues in the upper half of the array.
6. This process repeats, halving the search space each time, until the target is found or it's determined that the target is not in the array.

Binary search is much faster than linear search for large datasets, with a time complexity of O(log n) compared to O(n) for linear search. 🚀

Let's write the function that implements the preceding steps:
```rust,ignore
{{#include ./binary_search.rs:search_function}}
```

Here, we created a function that has the same input and output as the linear search algorithm. The only thing that has changed is the internal working

Two variables `low` and `high` are initialed to extreme points of the array. We then format a while loop which terminates when high is smaller than low. This suggests that we have been through the entire array and haven't found the `target` element.

In the loop, we initialize a `mid` variable which is the mid point of array to be searched in. If the `target` compared with element in mid index is equal, we return `Some(mid)`. If not then based on whether the element is smaller or larger than `target` we either reassign high to mid (new array will be on left side of `mid`), or low to mid (new array will be on the right side of `mid`). The visuals below will help.

<!-- !TODO Add diagrams showing the journey of binary search -->


### Result 📦

```rust,editable
{{#include ./binary_search.rs:binary_search}}
```

### Time and Space Complexity

The time complexity of binary search is O(log n), where n is the number of elements in the array. This is because the algorithm divides the search space in half with each iteration, effectively reducing the number of comparisons needed to find the target element.

The space complexity of binary search is O(1), as it does not require any extra space other than the input array and the target.
<br>
<br>
<hr>

Try this [LeetCode Problem](https://leetcode.com/problems/binary-search/description/).
