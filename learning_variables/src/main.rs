fn main() {

    println!("--- EXPLICIT QUANTITATIVE -------------------------------------");
    let mut x: i8 = 5; // i32 is signed therefore it can be negative, unsigned int types cannot have a sign (like -) so they can ONLY be positive
    let my_int: i32 = 13; // i32 is the default for integers
    let my_float: f64 = 3.14; // f64 is the default for floating point numbers

    println!("The value of my_int is: {my_int}");
    println!("The value of my_float is: {my_float}");
    

    println!("--- NUMERICAL OPERATIONS -------------------------------------");
    // numerical operations

    // addition
    let sum = 5+10;
    println!("The sum is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The difference is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The product is: {product}");

    // division
    let quotient = 39.4/12.8;
    println!("The quotient is: {quotient}");
    let truncated = -5 / 3; // results in -1;

    // remainder
    let remainder = 43 % 5;
    println!("The remainder is: {remainder}");

    

    // booleans 
    let t = true;
    let f: bool = false; // explicit type annotation

    // chars

    println!("--- CHARS -------------------------------------");

    // chars use single quotes '' & are rust's most primitive datatype

    let c = 'z';
    let z:char = 'Z'; // explicit type annotation
    let smiley_emoji = '😃';

    println!("{c}");
    println!("{z}");
    println!("{smiley_emoji}");

    
    println!("--- TUPLES -------------------------------------");

    // compound types

    // allows us to group together multiple values of different types into one single type

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x,y,z) = tup; // deconstructing tup and breaking it down into three seperate variables (x,y, and z)

    println!("The value of y is: {y}");
    

    println!("--- ARRAYS -------------------------------------");

    // arrays

    // arrays in rust have a fixed length, once declared, their size cannot be changed.

    // creating arrays
    let my_array = [1,2,3,4,5];
    let array: [i32;5] = [9,4,8,1,3]; // explicit type annotation

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let a = [3; 5]; // creates an array of 5 elements with each element initially being set to 3

    // accessing arrays

    let some_array = [1,2,3,4,5];

    let first = some_array[0]; // because arrays start at 0
    let third = some_array[2];

    println!("The first element in some_array is: {first}");
    println!("The third element in some_array is: {third}");


}
