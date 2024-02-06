// generic types in structs
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}
// Generic on Method
impl<T> Point<T> {
    fn x(&self) {
        &self.x
    }
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

// concrete type constraints
// Only Point<f32> will have access to this method
// meaning for example an intiaized Point<u32>.distance_from_origin()
// would be invalid
impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);

    println!("The largest char is {}", result);

    let struct_int = Point { x: 5, y: 10 };
    let struct_float = Point { x: 1.0, y: 4.0 };
    let mixture = Point { x: 15, y: 12.5 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// Generic function
// in order to do a comparison we need to restrict the T type
// use the PartialOrd trait
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// instead of duplicating functions
// lets use generics

//fn largest_i32(list: &[i32]) -> &i32 {
//    let mut largest = &list[0];
//
//    for item in list {
//        if item > largest {
//            largest = item;
//        }
//    }
//    largest
//}
//
//fn largest_char(list: &[char]) -> &char {
//    let mut largest = &list[0];
//
//    for item in list {
//        if item > largest {
//            largest = item;
//        }
//    }
//    largest
//}
