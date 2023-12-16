fn main() {
    loop {
        let mut userlimit = String::new();

        // prompt user
        println!("enter a index number and we'll print the coresponding fibonacci number (ex. 3 -> 1, 1, 2)");
        println!("enter 'q' to quit");
        // ask user for number
        io::stdin.read_line(&mut userlimit).expect("failed to read in number!")

        // let the user quit
        if userlimit == 'q' {
            std::process::exit(0);
        }
        // convert to i32
        let mut userlimit = match userlimit.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        // add 1 since index of array starts at zero
        let userlimit = userlimit + 1

        // pass into the fibcalc function
        let seq = fibcalc(userlimit);

        // print the numbers in the seq to stdout
        for f in seq {
            print!("{}, ", f);
        }
    }
    // array of size specified -1 (index begins at zero)
}

// fibocalc - calcualte numbers up to user specified number
// return them in an array
fn fibocal(limit: i32) -> [i32; size] {
    let mut n = 0;
    
}
