fn main() {
    loop {
        let mut userlimit = String::new();
        // ask user for number:
        io::stdin.read_line(&mut userlimit).expect("failed to read in number!")

        // convert to i32
        let userlimit = match userlimit.trim() {
            Ok(val) => val,
            Err(_) => continue,
        }
    }
    // array of size specified -1 (index begins at zero)
}

// fibocalc - calcualte numbers up to user specified number
// return them in an array
fn fibocal(limit: i32) -> [i32; size] {

}
