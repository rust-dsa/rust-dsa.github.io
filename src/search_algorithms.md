# Search algorithms

An important and recurring problem in computing is locating information. More succinctly, this problem is called as searching. To solve this problem, search algorithms are used.

As you may have guessed from the name of the algorithms, search algorithms help us locate data in data structures.


In order to simplify matters, we'll search through items in a array. Recalling what we learned in chapter 2, arrays are a collection of similar items stored in a contiguous location. Searching through it will be simple enough.

Let's discuss what a search algorithm will take as input and will give out as output.

* Input: 
   1. the array
   2. element to be searched

* Output:
   1. `Some(index)` if element is present in the array
   2. `None` if element is not present in the array


## Linear search algorithm
The most straightforward approach to searching is to examine each element of the data structure sequentially. Concurrently this searching algorithm is called linear search or sequential search.

> Imagine you're in a library, searching for a book by your favorite thriller author. You head to the thriller section and start scanning the shelves. You examine each book, one by one, until you find the one you're looking for. This process is akin to a linear search. There are a few possible outcomes: you might find the book you want, reach the end of the section without finding it, or decide to continue searching even after finding one book by the author, looking for additional titles.

### Algorithm

1. The algorithm takes the array and the token (number to be searched) as input.
2. It compares the token with every element in the array.
3. If the token and element match, the algorithm returns the index of that element.
4. If the token doesn't match with any element (i.e., it reaches the end of the array), it returns `None`.

Now let's write a function that implements the preceding steps:

```rust,ignore
{{#include ./code/algo/linear_search.rs:search_function}}
```

Here, we created a function that accepts an array and the element to be searched and gives out the Option enum.

Inside the function, a `for` loop iterates through every element in the array, that element is compared with given token for equality. If equal, the index of the element enclosed inside `Some` is returned.  When the loop iterates through the array and cannot find the token, then the function returns `None`.

Now that we have a function let's give it the inputs
```rust,ignore
{{#include ./code/algo/linear_search.rs:input}}
```
Note that the by default type of both these variables is `Vec<i32>` and `i32` respectively.

By setting up the variables and the outputs in the main function

```rust
{{#include ./code/algo/linear_search.rs:main}}
```

### Result 📦

```rust,editable
{{#include ./code/algo/linear_search.rs:linear_search}}
```

### Time and Space Complexity

The time complexity of linear search is O(n), where n is the number of elements in the array. This is because in the worst case, the algorithm will have to compare the token with every element in the array.

The space complexity of linear search is O(1), as it does not require any extra space other than the input array and the token.

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
{{#include ./code/algo/binary_search.rs:search_function}}
```

Here, we created a function that has the same input and output as the linear search algorithm. The only thing that has changed is the internal working

Two variables `low` and `high` are initialed to extreme points of the array. We then format a while loop which terminates when high is smaller than low. This suggests that we have been through the entire array and haven't found the `target` element.

In the loop, we initialize a `mid` variable which is the mid point of array to be searched in. If the `target` compared with element in mid index is equal, we return `Some(mid)`. If not then based on whether the element is smaller or larger than `target` we either reassign high to mid (new array will be on left side of `mid`), or low to mid (new array will be on the right side of `mid`). The visuals below will help.

<!-- !TODO Add diagrams showing the journey of binary search -->


### Result 📦

```rust,editable
{{#include ./code/algo/binary_search.rs:binary_search}}
```

### Time and Space Complexity

The time complexity of binary search is O(log n), where n is the number of elements in the array. This is because the algorithm divides the search space in half with each iteration, effectively reducing the number of comparisons needed to find the target element.

The space complexity of binary search is O(1), as it does not require any extra space other than the input array and the target.
<hr>

> Try this [LeetCode Problem](https://leetcode.com/problems/binary-search/description/).
