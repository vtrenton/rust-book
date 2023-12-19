use std::io;

fn main() {
    loop {
        let mut userlimit = String::new();

        // prompt user
        println!("enter a index number and we'll print the coresponding fibonacci number (ex. 3 -> 1, 1, 2)");
        println!("enter 'q' to quit");
        // ask user for number
        io::stdin().read_line(&mut userlimit).expect("failed to read in number!");

        // let the user quit
        if userlimit == "q" {
            std::process::exit(0);
        }
        // convert to u32
        let mut userlimit: u32 = match userlimit.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        // user input should not change past this point
        const USRSIZE: u32 = userlimit;

        // pass into the fibcalc function
        let seq = fibocal::<USRSIZE>();

        // print the numbers in the seq to stdout
        for f in seq {
            print!("{}, ", f);
        }
    }
}

// fibocalc - calcualte numbers up to user specified number
// return them in an array
fn fibocal<const SIZE: u32>() -> [u32; size] {

    // set up array we will populate with caller
    let mut arr = [u32; size];

    let mut a = 0;
    let mut b = 1;

    if SIZE <= 0 {
        return [0];
    }
    
    for i in 2..SIZE {
        let c = a + b;
        let a = b;
        let b = c;

        // append value to array - how am i going to do this

    }

}
