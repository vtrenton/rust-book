enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum method
impl Message {
    fn call(&self) {
        println!("Printing message...");
        dbg!(self);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //...
    Zebra,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn main() {
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));
    
    let m = Message::Write(String::from("Hello!"));
    m.call();

    let some_number = Some(5);
    let some_letter = Some('e');

    let absent_number: Option<i32> = None;

    // Run the value_in_cents Associated Function
    // and pass in the Coin::Quater which takes
    // a UsState - so we pass in UsState::Alaska
    value_in_cents(Coin::Quarter(UsState::Alaska));

    // Option enum
    let five = Some(5);
    let six = plus_one(five);

    // handle a "None"
    let none = plus_one(None);

    let config_max = Some(3u8);
    // This can be rewriten to remove the catch-all boiler plate
    //match config_max {
    //    Some(max) => println!("The max is {}", max);
    //    _ => () // this is a catch all
    //}
    //
    // This is the if let approach
    // it evaluates Some(max) and determines if
    // it needs to enter the block
    // The if let approach is not exhaustive so match can be safer
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    // Match is like a case/swtich statement
    // for enum values
    // A Coin called coin comes in
    // the type is matched and the u8 is returned
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        } // comma is optional on blocks
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The state of the Quarter is {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // Matching against an enum is exhaustive
    // meaning all possible varients must be
    // accounted for. IT would be impossible
    // to leave out None for example
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
