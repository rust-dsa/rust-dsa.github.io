pub fn vector_array() {
    //* INITIALIZING A VECTOR
    
    // ANCHOR: initialize
    // vector from inbuilt macro
    let vec:Vec <u16> = vec![10, 20, 30, 40, 50];
    println!("Vector array: {:?}", vec);

    // vector from array
    let vec = Vec::from([1, 2, 3, 4, 5]);
    println!("Vector array: {:?}", vec);

    // empty vector
    let mut _vec: Vec<u8> = Vec::new();
    // ANCHOR_END: initialize

    //* ACCESSING AND MODIFYING A VECTOR

    // ANCHOR: access_and_modify
    let mut vec = vec![10, 20, 34, 40, 50];
    // accessing an element
    let mid = vec[2];
    println!("Mid element: {mid}");

    // modifying an element
    vec[2] = 30;
    println!("Modified vector: {:?}", vec);
    // ANCHOR_END: access_and_modify


    // inserting and removing an element
    let mut vec = vec![12, 23, 34, 45];
    vec.push(56); // push function will insert the element at the end of vec
    println!("{:?}", vec);

    vec.pop(); // pop function will remove the element at the end of vec
    println!("{:?}", vec);

    //* INITIALIZE A 2D VECTOR

    // ANCHOR: 2D_vector
    // empty 2D vector
    let mut _vec: Vec<Vec<char>> = Vec::new();

    // method 1:
    let vec: Vec<Vec<u8>> = Vec::from([Vec::from([1, 2, 3]), Vec::from([4, 5, 6])]);
    println!("2d vector: {:?}", vec);

    // method 2:
    let vec: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
    println!("2d vector: {:?}", vec);
    // ANCHOR_END: 2D_vector

}