pub fn primitive_array() {
    //* DEFINING AN ARRAY
    // ANCHOR: initialize
    // default type is i32
    let arr = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
    // ANCHOR_END: initialize

    // ANCHOR: quantify
    // name: [T ; N] = [...
    let arr: [u8; 3] = [0, 3, 5]; 
    println!("Array: {:?}", arr);
    // ANCHOR_END: quantify

    // ANCHOR: repeat
    // name: [T ; N] = [E; N];
    let arr: [u8; 4] = [9; 4]; 
    println!("Array: {:?}", arr); 
    // ANCHOR_END: repeat

    //* MODIFYING AN ARRAY
    
    // array_name[index] = new_value;
    let mut arr1_mut = [12, 20, 30, 40];
    println!("Mutable array 1: {:?}", arr1_mut);
    arr1_mut[0]= 10;
    println!("Mutable array 1: {:?}", arr1_mut);


    //* TWO DIMENSIONAL ARRAY

    // ANCHOR: 2D_array
    const ROWS: usize = 4;
    const COLS: usize = 5;

    // name: [[ T;    C];    R]
    let arr: [[u8; COLS]; ROWS] = [[1; COLS]; ROWS];
    println!("Array: {:?}", arr); 
    // ANCHOR_END: 2D_array

    //* ACCESSING A TWO DIMENSIONAL ARRAY

    // ANCHOR: 2D_array_access
    const R: usize = 4;
    const C: usize = 5;

    let arr: [[u8; C]; R] = [[1; C]; R];
    let arr_element = arr[3][4];
    println!("Array: {}", arr_element); 
    // ANCHOR_END: 2D_array_access

}

