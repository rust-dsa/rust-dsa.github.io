# Search algorithms

An important and recurring problem in computing is locating information. This problem is called as searching. To solve this problem, search algorithms are used.

As you may have guessed from the name of the algorithms, search algorithms help us locate data in data structures.

In order to simplify matters, we'll search through items in a array. Recalling what we learned in chapter 2, arrays are a collection of similar items stored in a contiguous location. Searching through it will be simple enough.

Let's discuss what a search algorithm will take as input and will give out as output.

* Input:
   1. the array
   2. element to be searched

* Output:
   1. `Some(index)` if element is present in the array
   2. `None` if element is not present in the array