struct ImportExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // third elision rule
    // rust can infer the lifetimes
    // the return will have the lifetime of &self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // this works because we are referencing 
    // all resources within their lifetimes
    let str1 = String::from("long string is long");
    {
        let str2 = String::from("xyz");
        let result = longest(str1.as_str(), str2.as_str());
        println!("The longest string is {}", result);
    }
    // but if I move println here
    // the lifetime of result and str2
    // ended before they could be used in println!
    //println!("The longest string is {}", result);
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentance = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportExcerpt {
        part: first_sentance,
    };
    println!("{}", i.part);

    // static lifetime - life of the program
    let s: &'static str = "I have a static lifetime.";
}

// lifetime annotations
// there is no way to determine which will return x or y
// either could so their lifetimes can't be predicted
// thus we can annontate everything to have the same lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
