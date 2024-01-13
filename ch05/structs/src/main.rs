 struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
 }

// tuple scruct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit-like struct
struct AlwaysEqual;

fn main() {
    // instantiate a new 'User' from the Struct
    let mut user1 = User {
        // Key : value defintions
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // thank the lawd rust uses dot notation
    // Note: the entire instance of the
    // struct needs to be mutable for this to work
    user1.email = String::from("anotheremail@example.com");

    // Struct update syntax
    //let user2 = User {
    //    active: user1.active,
    //    username: user1.username,
    //    email: String::from("another@example.com"),
    //    sign_in_count: user1.sign_in_count,
    //};

    // The above can be rewritten as
    // Note: the values copied into user2 are
    // no longer accessible in user1 now
    let user2 = User {
        email: String::from("another@example.com"),
        // must be the last field
        ..user1
    };

    // interact with tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // instance of unit-like struct
    let subject = AlwaysEqual;
}

// A function to take in a username and email then return a User instance
fn getUser(username: String, email: String) -> User {
    User {
        active: true,
        // replace with field init shorthand
        // I can do this because the param names
        // and the field names are the same.
        //username: username,
        //email: email,
        username,
        email,
        sign_in_count: 1,
    }
}
