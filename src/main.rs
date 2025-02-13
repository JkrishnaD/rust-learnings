use core::f64;
use std::{collections::HashMap, fs};
struct User {
    name: String,
    email: String,
    password: String,
    is_active: bool,
}
struct Rect {
    width: u32,
    height: u32,
}
// struct implements
impl Rect {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn perimeter(&self) -> u32 {
        2 * (self.height * self.width)
    }
}

//enums
enum Directions {
    North,
    South,
    East,
    West,
}

enum Shapes {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}
fn main() {
    println!("Hello, world!");

    // variables and datatypes
    let x: u32 = 5;
    let y = -2;
    let z = 0.1;

    println!("x {} ,y {} , z{}", x, y, z);

    let str = String::from("hello world!");
    println!("{}", str);

    let char1 = str.chars().nth(100);
    // we can't directly print the chracters becaues it is optional so we can't compile
    // so we use the pattern matching
    match char1 {
        Some(c) => println!("{}", c),
        None => println!("no charac at 1000"),
    }

    let arr = [1, 2, 3, 4, 5];
    println!("arr len {}", arr.len());

    let sentence = String::from("jaya is here");
    let first_word = get_first_word(sentence);
    println!("first word is {}", first_word);

    let a = 10;
    let b = 20;
    let their_sum = do_sum(a, b);
    println!("sum is {}", their_sum);

    //vectors
    let mut xs = vec![1, 2, 3];
    println!("vector lenght {}", xs.len());
    xs.push(4);
    println!("modified vector lenght {}", xs.len());
    println!("vector is {:?}", xs); // syntax to print the vector in rust

    // conditions
    let num = 90;
    let is_even = find_even(90);
    if is_even {
        println!("{} is even number", num);
    } else {
        println!("{} is odd number", num);
    }

    // int loop
    for i in 0..100 {
        let _num = i + 1;
        // println!("{}", num);
    }

    let s1 = String::from("jayakrishna");
    let s2 = take_ownership(s1); // ownership moves to s2 and s1 is out of scope

    let s1 = take_ownership(s2); // ownership moves back to s1 and here s2 is out of scope

    println!("{}", s1);
    borrow_variable(&s1); // refering to the s1 here there is no ownership change and we can borrow multiple times
    let s3 = &s1; // this is also refering
    println!("{} {}", s1, s3);

    let mut s4 = String::from("jayakrishna");
    update_str(&mut s4);
    let s5 = &mut s4;
    // let s6 = &mut s4; // s5 is already has mutable reference so it throws the error for the other.
    // we can't have more than one mutable reference at the same time.
    // but we can have the mutliple borrowers.
    println!("{}", s5);

    let user1 = User {
        name: String::from("jaya"),
        email: String::from("jayak5063@gmail.com"),
        password: String::from("******"),
        is_active: true,
    };

    println!("{} is name and {} is email", user1.name, user1.email);

    let rect = Rect {
        width: 30,
        height: 20,
    };
    println!("area of the rectangle is {}", rect.area());
    println!("perimeter of the rectangle is {}", rect.perimeter());

    //accessing enums
    let my_direction = Directions::North;

    // calculating by the use of the pattern matching
    let circle = Shapes::Circle(5.0);
    let rectangle = Shapes::Rectangle(6.0, 4.0);
    let square = Shapes::Square(4.0);

    let circle_area = calculate_area(circle);
    let rectangle_area = calculate_area(rectangle);
    let square_area = calculate_area(square);

    println!("{}", circle_area);
    println!("{}", rectangle_area);
    println!("{}", square_area);

    //error handling using Result enum
    let res = fs::read_to_string("path");
    match res {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            println!("error occured {:?}", err);
        }
    }
    //vectors

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{:?}", even_vector(vec));

    // hashmaps
    let input_vec = vec![(String::from("jayak"),20),(String::from("krishna"),19)];
    println!("{:?}",input_vec); // printing the hashmap
    let hm = get_values_by_keys(input_vec);
    println!("{:?}",hm);
}

// functions
fn find_even(x: i32) -> bool {
    return x % 2 == 0;
}

fn do_sum(a: i32, b: i32) -> i32 {
    return a + b;
}

//str loops
fn get_first_word(sentence: String) -> String {
    let mut ans = String::new();
    // this is a way for string iterations
    for char in sentence.chars() {
        if char == ' ' {
            break;
        }
        ans.push(char);
    }
    return ans;
}

fn update_str(s: &mut String) {
    s.push_str(" is smart");
}

// passing ownership
fn take_ownership(some_string: String) -> String {
    println!("{} is the string from ownership", some_string);
    return some_string;
}
// borrowing
fn borrow_variable(some_string: &String) {
    println!("borrow variable is {}", some_string)
}

// implementing the pattern matching
fn calculate_area(shape: Shapes) -> f64 {
    let ans = match shape {
        Shapes::Circle(radius) => 3.14 * radius * radius,
        Shapes::Rectangle(width, height) => { width * height } // corrected pattern
        Shapes::Square(side) => side * side,
    };

    return ans;
}
// finding the even numbers in vectors
fn even_vector(v: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in v {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }
    return new_vec;
}

fn get_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }
    return hm;
}
