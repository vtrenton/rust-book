fn main() {
    let s = String::from("Hello"); // s comes into scope

    takes_ownership(s); // s now is passed into takes_ownership
                        // s has now changed scope - it can no longer be referenced
    // this would fail with a borrow after move
    //println!("{}", s);

    let x = 5; // an i32 comes into scope
               // i32 implements the copy
    
    makes_copy(x); // pass x into the function
    
    // totally valid
    println!("after copy the value is still here local to main: {}", x);

    let s1 = gives_ownership(); // gives_ownership sends its return value to s1
    let s2 = String::from("Hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved to the takes_and_gives_back function
                                        // s2 is out of scope
                                        // s3 is assigned to the return value

    // we can return multiple values as a tuple
    let str1 = String::from("Hello");
    let (str2, len) = calculate_length(str1);

    println!("The len of '{}' is {}.", str2, len).

} // here x goes out of scope, s has already been moved into the take_ownership function

fn takes_ownership(some_string: String) {
    println!("value inside of take_ownership function: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("here is the copy of the i32 in the make_copy function: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // return the String to the caller to be assigned as s1 in main
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // return s2 as is back to s3 in main
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // calculate the length and store it in local var

    (s, length) // return 's' as is but also return the length 'usize'
}