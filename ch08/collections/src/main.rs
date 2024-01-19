fn main() {
    // intiialize a new vector
    let v: Vec<i32> = Vec::new();
    // vector macro
    let v = vec![1, 2, 3];
    // create a mutable vector and assign data to it
    let mut v = Vec::new();

    for i in 1..10 {
        v.push(i);
    }

    // get the values out of the vector
  
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    
    // using an option lets us catch errors for out of bound access
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    
    // PANIC
    //let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    
    // this doesnt work because of the way vectors work in memory

    let first = &v[0];
    
    // when venctors are resized they are copied 
    // to a new location in memory to remain contigous
    // this mutates the Vector

    v.push(22);

    // so after this point the value of 'first' could
    // be pointing to a dereferenced value in memory

    //println!("The first element is {first}");

    // we iterate over each element to get an immutible reference
    for i in &v {
        println!("{i}");
    }
    
    // we can add to mutable vectors with the following
    let mut mvec = vec![100, 32, 57];
    for i in &mut mvec {
        // in order to mutate mvec
        // we need to dereferce the mutation using *
        // This gets the value at the element and adds 50
        *i += 50;
    }

    for i in mvec {
        println!("{i}");
    }

    // using a enum we can store multiple types of values
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    // This is a vector of SpreadsheetCells
    // as far as the vector is concerned this is all the same type
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    {
        let g = vec![1, 2, 3, 4];
    } // Vector leaves scope here
      // Vector is freed

    // Strings!!!!
    
    // this is a string literal
    // the inferred type is &str
    // a borrowed str
    let data = "initial contents";
    // show type

    // I can convert it to a String with the to_string method
    let s = data.to_string();

    // I can also use this on the literal directly
    let s = "initial contents".to_string();

    // these are valid Strings!
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // We can concatinate Strings together with push_str
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");
   
    // we can retain the value of the orginal string as well
    // push_str doesnt take ownership of s2
    // this means we can pass it in without borrowing
    // and still use it in the same scope
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    println!("s1 is {s1}");

    // push only appends chars
    let mut lol = String::from("lo");
    lol.push('l');

    // we can concatinate strings using the + as well
    let hello = String::from("Hello, ");
    let world = String::from("World");
    let hw = hello + &world; // note that hello is moved
                             // hello can no longer be used
    // '+' uses the add method
    // the signature of the add method is as follows
    // fn add(self, s: &str) -> String
    // so we passed in a reference to world to the 'add' method

    // not that add could take in a &String instead of &str 
    // This is because the value is coerced from &String -> &str
    
    //we can think about a concatenation like this
    //let hw = hello.add(&world);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // for multiple concats we use the format! macro
    let s = format!("{s1}-{s2}-{s3}");

    // rust cannot index Strings
    //let mystr = String::from("hello");
    //let h = s1[0];
     
    //Rust has a similar issue with Go Runes
    let hello = String::from("Hola"); // is 4 bytes - so the len is 4
    let hello = String::from("Здравствуйте"); // is 24 bytes - each UTF-8 scalar
                                              // value is encoded with 2 bytes

    // UTF-8 has 3 relevant ways to looks at strings
    // bytes, Scalar Values (chars), grapheme clusters (what we call letters)

    // looking at the hindi word नमस्ते
    //
    // in bytes this is represented as
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    //
    // encoded as chars this is represented as
    // ['न', 'म', 'स', '्', 'त', 'े']
    // the forth and sixth values are not letters - they're diacritics or accents on letters
    //
    // finally - as a grapheme clusters
    // ["न", "म", "स्", "ते"]
    
    // This will return Зд because each letter is 2 bytes
    // a string slice will return a specified number of bytes
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // This would panic
    //let s =  &hello[..1];
    // because this cannot be converted to a char
    
    // to convert the input to chars we can
    for c in "Зд".chars() {
        println!("{c}");
    }

    // to convert the input to bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Hash Maps

    // HashMaps need to be brought into scope with use
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    
    // insert values into a hashmap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // All keys must be the same type
    // All Values must be the same type
    // Keys and Values do not need to be the same type
    
    // to access data out of a hashmap we can use get
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // HashMap.get(&ref) returns a Option<&V> so it can handle a none condition
    // Option<&i32>.copied() will return an Option<i32> (unrefed value
    // unwrap_or will handle the None condition and set it it 0

    // To iterate over a hashmap we can use for loops
    // note that a hashmap reference contains Tuples
    // so we use a tuple in the loop
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Types that implement Copy such as i32 are borrowed
    // Whereas Strings are owned by the HashMap
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // both field_name and field_value are no longer valid
    // as they are owned by the HashMap map
    // this would throw an ownership error
    //println!("{field_name}: {field_value}");
    
    // overwriting values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    // Blue is now 25
    
    println!("{:?}", scores);

    // We can use the HashMap.entry().or_insert() to only update
    // if the value is not set
    // else ignore updates
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(75);
    scores.entry(String::from("Purple")).or_insert(75);

    // This will not modify blue or yellow
    // but will create an entry for Purple
    println!("{:?}", scores);

    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    
    // split_whitespace() returns an iterator over slices
    for word in text.split_whitespace() {
        // or_insert returns a mutable ref &mut V
        // to the Value of the key

        // so with the first word "Hello"
        // this would not be found in the hashmap
        // so the or insert would create the key: value of
        // "Hello": 0
        let count = map.entry(word).or_insert(0);
        // every word obviously exists at least once
        // so we add one, in our example above this would update
        // "Hello": 1
        *count += 1;
    }

    println!("{:?}", map);
}
