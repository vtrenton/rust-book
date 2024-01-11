fn main() {
    let s1 = String::from("hello");
    // & is a reference to
    // this keeps s1 from leaving scope
    // s1 is 'borrowed' - the calling function maintains ownership
    let s2 = calculate_length(&s1);

    // I can access both s1 and s2
    // both are still in the main scope.
    println!("The len of '{}' is {}.", s1, s2);

    // in order for our program to mutate this later we need to declare it as mutable
    let mut s = String::from("Hello");

    // here we pass in a mutable reference
    change(&mut s);

    // A single reference at a time per variable in a given scope
    // this would not be allowed
    //let r1 = &mut s;
    //let r2 = &mut s;

    // I can however create as many immutable references as id like
    let r1 = &s;
    let r2 = &s;
    // But once declared as immutable I can no longer assign an mutable reference
    // This will not work
    //let r3 = &mut s; 

    println!("{} {}", r1, r2);

    // however the scope of the immutible references end once they are called for the last time in the function
    // this will work because printf has already called r1 and r2 and they will not be used after this
    let r3 = %mut s;

    println!("{}", r3);

    let reference_to_nothing = no_dangle();

}

// we declare the local variable as the type of 'reference' to a String
fn calculate_length(s: &String) -> usize {
    // s is a stack variable now pointing to
    // s1 which is pointing to the String ([]char) on the heap
    /*
    +-------------------+    +-------------------+
    | Variable `s`      |    | Variable `s1`     |
    |-------------------|    |-------------------|
    | Pointer ----------|--> | Stack Data: s1    |
    |                   |    | Pointer ----------|--> [String Data on Heap]
    +-------------------+    +-------------------+
    */
    
    // return only the length
    s.len()
}

// the type of some string is a mutable reference to a string
// variables and references are immutible by default
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
} // the scope of the borrowed mutable s ends here
  
// this will not run
//fn dangle() -> &String { // dangle returns a ref to a string
//    let s = String::from("hello"); // create a new string

//    &s //return the reference to the String, s
//} // s no longer exists, s is dropped
  // once its dropped the string reference would no longer be pointing a string

// instead we should pass the String not the reference
fn no_dangle() -> String {
    let s = String::from("Hello");

    s
}

// references
// At any time you can only have one mutable reference or any number of immutible refences
// references must point to a valid value
