use std::{
    fs::{self, File},
    error::Error,
    io::{self, ErrorKind, Read},
    net::IpAddr
};

// main can return an Result
// the normal return type of main is ()
// but we can wrap it in a Result
fn main() -> Result<(), Box<dyn Error>> {
    // Open file
    //let greeting_file_result = File::open("hello.txt");
    
    // check result of opening file
    //let greeting_file = match greeting_file_result {
        //Ok(file) => file,
        // On Error check the kind of error
        //Err(error) =>  match error.kind() {
            // determine if its a NotFound
            // if so create the file
            //ErrorKind::NotFound => match File::create("hello.txt") {
                //Ok(fc) => fc,
                //Err(e) => panic!("Problem creating the file");
            //},
            //other_error => {
                //panic!("Problem opening the file: {:?}", error),
            //}
        //},
    //};
    
    // Better method using closures and unwrap_or_else

    // open the file
    // if it fails to unwrap catch the error
    // pass as a parameter of an anonymous function
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        // check to see if the kind of Error is NotFound
        if error.kind() == ErrorKind::NotFound {
            // create the file
            // if it fails to unwrap catch the error
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            // catch-all for non NotFound Errors
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Result<T, E> has helper methods

    // unwrap will panic automatically if error is returned
    let another_file = File::open("other.txt").unwrap();

    // expect is like unwrap but
    // gives the ability to set the error message
    let blue_file = File::open("blue.txt")
        .expect("blue.txt should be included in this project");

    // This is brokd
    //let greeting_file = File::open("hello.txt");
    
    // Return the Result to main
    let greeting_file = File::open("hello.txt")?;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP Address should be valid");
    
    // If all goes well return the defualt return of main
    Ok(())

}

fn read_username_from_file() -> Result<String, io::Error> {

    //let mut username = String::new();
    // instead of manually writing a match we can use the 
    // question mark '?' to return the Result to the caller
    // we can even chain with the ?
    //File::open("username.txt")?.username_file.read_to_string(&mut username)?;

    //let mut username_file = match username_file_result {
        //Ok(file) => file,
        //Err(e) => return Err(e),
    //};

    // If all goes well we return the username
    // wrapped as Ok result
    //Ok(username)
    
    // This returns the result
    // notice there is no semicolon at the end of the statement
    // We also dont use return because this is
    // the last expression in the function
    //match username_file.read_to_string(&mut username) {
        //Ok(_) => Ok(username),
        //Err(e) => Err(e),
    //}
    
    // The apex alpha-chad way to shorten this function
    fs::read_to_string("username.txt")

}

fn last_char_of_the_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


