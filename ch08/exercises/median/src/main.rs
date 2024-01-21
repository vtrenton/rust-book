use std::{collections::HashMap, process};

fn main() {
    let mut list = vec![1, 3, 4, 15, 18, 3, 2, 12, 18, 22, 18];
    // exit if Vector is empty
    if list.is_empty() {
        println!("The Vector is empty!");
        process::exit(1);
    }
    
    // Sort the list before processing
    list.sort();
    //dbg!(&list);
    println!("This list is {:?}", list);

    // Get the Median of the list
    let median_index = get_median(&list);

    match list.get(median_index) {
        Some(value) => println!("The median is {}", value),
        None => println!("The index is out of range!"),
    }

    // Get the Mode of the list
    let (int, count) = get_mode(&list);
    println!("The mode is {int} with a count of {count}");
}

fn get_median(list: &Vec<i32>) -> usize {
    let len = list.len();
    if len == 1 {
        0
    }
    else {
        len / 2
    }
}

fn get_mode(list: &Vec<i32>) -> (i32, i32) {
    let mut mode_map = HashMap::new();
    for &i in list {
        let count = mode_map.entry(i).or_insert(0);
        *count += 1;
    }
    
    // create a vector of Tuples
    // iter() returns references to values
    // so we need to use map to deref them
    let mut mode_vec: Vec<(i32, i32)> = mode_map
        .iter()
        .map(|(&k, &v)| (k, v))
        .collect();
    
    // sort vector in descending order
    mode_vec.sort_by(|a, b| b.1.cmp(&a.1));

    //dbg!(&mode_vec);

    // get the first element of the vector and return it
    // this will be the mode or heightest value
    match mode_vec.get(0) {
        Some(mod_tup) => *mod_tup,
        None => {
            println!("Something went wrong!");
            process::exit(1);
        }
    }
}
