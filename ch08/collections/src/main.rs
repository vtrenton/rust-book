fn main() {
    // intiialize a new vector
    let v: Vec<i32> = Vec::new();
    // vector macro
    let v = vec![1, 2, 3];
    // create a mutable vector and assign data to it
    let mut v = Vec::new();

    for i in 1..10 {
        v.push(i);
    }

    // get the values out of the vector
  
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    
    // using an option lets us catch errors for out of bound access
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    
    // PANIC
    //let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    
    // this doesnt work because of the way vectors work in memory

    let first = &v[0];
    
    // when venctors are resized they are copied 
    // to a new location in memory to remain contigous
    // this mutates the Vector

    v.push(22);

    // so after this point the value of 'first' could
    // be pointing to a dereferenced value in memory

    //println!("The first element is {first}");

    // we iterate over each element to get an immutible reference
    for i in &v {
        println!("{i}");
    }
    
    // we can add to mutable vectors with the following
    let mut mvec = vec![100, 32, 57];
    for i in &mut mvec {
        // in order to mutate mvec
        // we need to dereferce the mutation using *
        // This gets the value at the element and adds 50
        *i += 50;
    }

    for i in mvec {
        println!("{i}");
    }

    // using a enum we can store multiple types of values
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    // This is a vector of SpreadsheetCells
    // as far as the vector is concerned this is all the same type
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    }
    
    {
        let g = vec![1, 2, 3, 4];
    } // Vector leaves scope here
      // Vector is freed

    // Strings!!!!

}
