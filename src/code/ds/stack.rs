fn stack() {
    //* INITIALIZING A STACK
    // ANCHOR: initialize
    let mut stack = Vec::new();
    // ANCHOR_END: initialize


    //* OPERATIONS ON A STACK

    // ANCHOR:  operations
    # let mut stack = Vec::new();
    // PUSH
    // name.push(element);

    stack.push(74);
    stack.push(83);
    stack.push(44);
    stack.push(91);

    println!("Stack: {:?}", stack);

    // POP
    // name.pop();
    stack.pop(); // pops 91
    
    // var_name = name.pop();
    let work = stack.pop().unwrap();
    println!("After pop: {work}");
    // ANCHOR_END: operations
    
    
    //* 
    // ANCHOR: 
    // ANCHOR_END:
}