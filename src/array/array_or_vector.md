## Array or Vector?
In most cases, vectors are considered better than arrays for a few key reasons:

- **Size flexibility:** Vectors are dynamic, meaning their size can grow or shrink as needed during program execution. Arrays have a fixed size defined at the time of creation, which can lead to problems if you don't know exactly how many elements you'll need beforehand.
- **Memory management:** Vectors handle memory allocation and deallocation automatically. Arrays require manual memory management, which can be error-prone and lead to memory leaks.
- **Built-in functionality:** Vectors often come with additional features like methods for sorting, searching, and element insertion/deletion at specific positions. Arrays typically require manual implementation for these operations.

However, there are some situations where arrays might be preferable:

- **Performance:** For fixed-size data sets, arrays can be slightly faster than vectors due to their simpler structure and lack of overhead for managing size changes.
- **Memory usage:** If you know the exact size you need upfront and memory usage is a critical concern, arrays can be a more memory-efficient choice.

While learning algorithms on arrays, we'll be using vectors as our array data structures because they are also easy to handle.