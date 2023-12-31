fn main() {
    let s = "Hello"; // string literal - string is hardcorded in the program

    // String type in rust is stored on the heap
    // This string can be mutated
    let mut s = String::from("Hello");
    s.push_str(", world!"); // push_str appends a literal to the string

    // prints Hello World
    println!("{}", s);

    // an int is a static predictible hard-coded value
    // this is done entirely on the stack
    let x = 5; // assign x to 5
    let y = x; // take the value of x and copy it into y

    // String handles this much differently
    let s1 = String::from("Hello");
    let s2 = s1;

    // The Stack contains a pointer to the heap
    // A size and capacity as well
    // on the heap contains the actual content of the String

    /*
    Stack               Heap
    +------+           +------+
    | s1   | ---->     | hello |
    | len:5|           +------+
    | cap:5|
    +------+
     */

    // len is the total bytes used by the string
    // cap is the total amount of memory in bytes allocated on the heap

    // when we assign s1 to s2 all of the string data 
    // on the stack is copied from s1 to s2
    // including the pointer to the data on the stack

    /*
    Stack               Heap
    +------+           +------+
    | s1   | ----+     | hello |
    | len:5|     |     +------+
    | cap:5|     |
    +------+     |
    | s2   | ----+
    | len:5|
    | cap:5|
    +------+
     */

    // if both s1 and s2 leave scope
    // since both are trying to free the same address
    // this leads to a "Double-Free error"

    // Rust will treat s1 as invalid after s2 is declared
    // So when s1 leaves scope - nothing will be done

    // This will break
    //println!("{}, World!", s1);

    // s1 is MOVED to s2
    // meaning only s2 is accessible
    // this is called a 'shallow copy'
    /*
    Stack               Heap
    +------+           +------+
    | s1   |           | hello |
    | len:5|           +------+
    | cap:5|           
    |(invalid)         
    +------+           
    | s2   | ---->     
    | len:5|           
    | cap:5|           
    +------+
    */

    // Rust will never perform "deep" copies automatically
    // this helps with runtitme performance as well as safety

    // If I do in fact want a 'deep copy'
    // I can use 'clone'
    let str1 = String::from('Hello');
    let str2 = str1.clone();    

    println!("str1 = {}, str2 = {}", str1, str2);

    // in memory the value on the heap is being reallocated as well:
    /*
    Stack               Heap
    +------+           +------+
    | str1 | ---->     | hello |
    | len:5|           +------+
    | cap:5|           
    +------+           +------+
    | str2 | ---->     | hello |
    | len:5|           +------+
    | cap:5|
    +------+
    */



}
