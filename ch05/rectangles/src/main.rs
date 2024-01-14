// enable Debugging for struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // the struct object of the method implemenation
    // is always passed in as a referece with the 'self' keyword
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // methods can have the same names as properties
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rec: &Rectangle) -> bool {
        // the books takes this approach
        //self.width > rec.width && self.height > rec.height
        // I'll take my own road :)
        // self.width * self.height > rec.width * rec.height
        self.area() > rec.area()
    }
}

fn main() {
    // refactor with tuples
    //let width1 = 30;
    //let height1 = 50;
    
    // lets try refactoring with a struct to make this more readable
    //let rec1 = (30, 50);
    //let scale = 2;
    let rec1 = Rectangle {
        width: 30, //dbg!(30 * scale),
        height: 50,
    };

    let rec2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rec3 = Rectangle {
        width: 60,
        height: 45,
    };


    println!(
        "The area of the rectangle is {} pixels",
        //area(&rec1)
        // lets use a method
        rec1.area()
    );

    // check our width is greater than zero
    if rec1.width() {
        println!("rec1 has a nonzero width, width is {}", rec1.width);
    }
    
    // debug statement
    // {:?} will pretty print objects on a single line
    // for multi-line i can use {:#?}
    //println!("rec1 is {:#?}", rec1);
    
    // instead of println which takes the object by reference
    // lets pass it into the dbg! macro
    // this will take ownership of the object
    // this will also print the line number!
    //dbg!(&rec1);
    // note this outputs to stderr (2) NOT stdout

    println!("can rec2 fit in rec1? {}", rec1.can_hold(&rec2));
    println!("can rec3 fit in rec1? {}", rec1.can_hold(&rec3));
}


// lets move this into a struct method
// rewrite this using tuples to better corelate data
//fn area(width: u32, height: u32) -> u32 {
//lets make this more readible with structs!
//fn area(dimensions: (u32, u32)) -> u32 {
//fn area(rectangle: &Rectangle) -> u32 {
    //width * height
    // We lose some readibility doing this
    //dimensions.0 * dimensions.1
    // how about this:
//    rectangle.width * rectangle.height
//}
