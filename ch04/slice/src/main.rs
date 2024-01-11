fn main() {
    // string slice
    let my_string = String::from("Hello World");
   
    // 'first word' works on Arrays of Strings whether partial or whole
    let word = first_word(&my_string[..6]);
    let word = first_word(&my_string[..]);
    // first word also works on references to Strings
    let word = first_word(&my_string);

    // Create a string literal
    let my_string_literal = "Hello World";

    // 'first word' works on slices of string literals
    let word = first_word(&my_string_literal[..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(&my_string_literal);

    // Array Slice
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, [2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    // bytes.iter().enumerate() returns 2 values in a tuple
    // an index and a reference to the value
    for (i, &item) in bytes.iter().enumerate() {
        // if the item is a space return the index
        if item == b' ' {
            &s[..i]
        }
    }
    // if we never find a space
    // return the entire string
    &s[..]
}
