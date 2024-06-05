#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    //String::from("Hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // We can use Results in tests
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    // example failure
    // #[test]
    // fn another() {
    //     panic!("Make this test fail!");
    // }
 
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));;
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        
        assert!(!smaller.can_hold(&larger));
    }

    // assert_eq to test equality
    #[test]
    fn adds_two() {
        assert_eq!(4, add_two(2));
        // the order of the params of asset_eq! dont matter
        // assert_eq!(add_two(2), 4);
        // is also valid
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    // this test will be ignored by default
    // ii can run it with a test bin flag
    // cargo test -- --ignored
    // or for all tests INCLUDING the ignored
    // cargo test -- --include-ignored
    #[test]
    #[ignore]
    fn expensive_test() {
        // very expensive test
    }


    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // we can add a custom failure message that contains the output.
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    // assure a panic
    // I can use expected to determine that I got the correct panic
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
