fn main() {
    let s1 = String::from("hello");
    // & is a reference to
    // this keeps s1 from leaving scope
    // s1 is 'borrowed' - the calling function maintains ownership
    let s2 = calculate_length(&s1);  

    println!("The len of '{}' is {}.", s1, s2);
}

// we declare the local variable as the type of 'reference' to the string
fn calculate_length(s: &String) -> usize {
    s.len()
}
