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
