# Array
An array is a collection  of similar data types which are stored in contiguous memory locations. Each location has an index number through which we can access the element of that particular index.
In programming, we have to use data elements of similar type and similar purpose. Instead of declaring and defining every variable individually, we can use an array to store all of them together.

In Rust an array can be defined in two methods. Each method comes with its pros and cons. The two types of array in rust are:

## Vector type
Unlike arrays, a vector stores the size of the array as well. Arrays don't need to store its size. That is why we can access an array element even when we exceed the actual capacity. The boundaries are not defined.
Another thing about vectors is they're generally stored in a heap and they have a large size allocated than being used

As arrays and vectors, both copy items into another larger array / vector once more element(s) are added, vectors do the copying less often as they already have more capacity than the actual length.

"A vector is a contiguous growable array type with heap allocated contents".

Capacity of a vector: The amount of space allocated for any further elements to be added onto the vector.
Length of a vector: The number of actual element present the vector.

> NOTE: <br>
> If length is greater than capacity (length > capacity) the vector is reallocated.

## Array or Vector?
In most cases, vectors are considered better than arrays for a few key reasons:

- **Size flexibility:** Vectors are dynamic, meaning their size can grow or shrink as needed during program execution. Arrays have a fixed size defined at the time of creation, which can lead to problems if you don't know exactly how many elements you'll need beforehand.
- **Memory management:** Vectors handle memory allocation and deallocation automatically. Arrays require manual memory management, which can be error-prone and lead to memory leaks.
- **Built-in functionality:** Vectors often come with additional features like methods for sorting, searching, and element insertion/deletion at specific positions. Arrays typically require manual implementation for these operations.

However, there are some situations where arrays might be preferable:

- **Performance:** For fixed-size data sets, arrays can be slightly faster than vectors due to their simpler structure and lack of overhead for managing size changes.
- **Memory usage:** If you know the exact size you need upfront and memory usage is a critical concern, arrays can be a more memory-efficient choice.

Keeping this in mind, let's look at some of the algorithms that go hand in hand with arrays

## Array Algorithms

### Search Algorithms 🔍
They serve the function of checking and retrieving information stored within some data structure where that data is stored.
#### 1. Linear Search algorithm
It is also called sequential search algorithm. This algorithm works by sequentially iterating through the whole array from one end until the target element is round. If found, the algorithm returns its index, else - 1.

#### 2. Binary Search Algorithm
This type of searching algorithm is used to find the position of a specific value contained in a sorted array. The binary search algorithm works on the principle of divide and conquer and is the best searching algorithm because its faster to run.


### Sorting Algorithms 🧲
Sorting algorithm rearrange a given array of elements in a specific order. In case of numbers, it could be ascending or descending order.

1.  Bubble Sort: It is the simplest sorting algorithm, where is iteratively swaps the adjacent algorithms if they are in the wrong order.
2.  Selection Sort: This algorithm works by iteratively selecting the smallest / largest element from the unsorted portion of the list and moving it to the sorted portion of the array.
3. Insertion sort: This sorting algorithm works by iteratively inserting each element of an unsorted array into its correct position in a sorted portion of the list. 
4. Merge sort: It is a sorting algorithm that follows the *divide and conquer* strategy. It works by iteratively dividing the input array into smaller subarrays and sorting those subarrays, then merging them back together to obtain the sorted array.
5. Quick sort: It is a sorting algorithm that implements the divide and conquer strategy and picks an element as a pivot and partitions the given array around the picked pivot by placing the pivot in its correct position in the sorted.


