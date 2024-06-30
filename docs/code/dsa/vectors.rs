pub fn vector_array() {
    // initializing a vector
    let vec = Vec::from([1, 2, 3, 4, 5]);
    println!("Vector array: {:?}", vec);

    let vec:Vec <u16> = vec![10, 20, 30, 40, 50];
    println!("Vector array: {:?}", vec);

    // inserting and removing an element
    // conditions: the vector has to be mutable
    let mut vec = vec![12, 23, 34, 45];
    vec.push(56); // push function will insert the element at the end of vec
    println!("{:?}", vec);

    vec.pop(); // pop function will remove the element at the end of vec
    println!("{:?}", vec);

}