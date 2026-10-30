use std::io;

fn main() {
    // rust will through an error if a type cannot be inferred
    // this will error: let guess = "42".parse().expect("Not a number!"); since the compiler cannot
    // infer what type we want. So, we do this:
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    // integers are signed (i32) or unsigned (u32) - u32 is the default
    // floats are signed and default to f64
    // all operations work how you think
    // integer division rounds towards the nearest 0

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // results in -1
    let remainder = 43 % 5;

    // BOOLS
    let t = true;
    let f: bool = false; // explicit type annotation

    // CHARACTER TYPE
    // characters are indicated with single quotes vs double quotes for strings
    let c = 'z';
    let z: char = 'Z'; //explicit type annotation
    let alien = '👽';

    // COMPOUND TYPES

    // the TUPLE - tuples have a fixed length but may contain a variety of types
    // the variable tup_var binds the entire tuple because a tuple is consider a single compound
    // element
    let tup_var: (i32, f64, f32) = (500, 6.4, 7.77);

    // to get individual elements out of a tuple, we can destructure it like so:
    let (t_1, t_2, t_3) = tup_var;

    println!("The value of the lines are {t_1}, {t_2}, {t_3}!");

    // we can also access a tuple element directly by using a period followed by the index we want
    // to access.
    let five_hundred = tup_var.0;
    let six_pt_four = tup_var.1;
    let sevens = tup_var.2;

    // arrays are fixed size (on the stack) collections of elements

    // the compiler infers strings from the double quotes
    let months = ["January", "February", "March", "April"];

    // you can also declare types explicit - note that we also list how many items are in the array
    let nums: [i32; 5] = [1, 2, 3, 4, 5];

    // we can also just initialize an array with the same repeated value with [val; num]
    let rep_nums = [3; 5];

    // we can access array elements using indexing

    let first = rep_nums[3];
    let second = nums[2];
    println!("{first} x {second} = 6!");

    // the compiler may not catch a user trying to access past the end of an array
    // to make this safe, rust will panic at runtime - try accessing array 10 when running below

    let arr = [1, 2, 3, 4, 5];

    println!("Please enter an array index!");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number!");

    let element = arr[index];

    println!("The value of the element as index {index} is: {element}");
}
