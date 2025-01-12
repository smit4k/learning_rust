fn main() {
    let number: i32 = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("Number was something other than 0")
    }

    // else ifs
    let my_num: i32 = 45;

    if my_num % 5 == 0 && my_num % 3 == 0 {
        println!("my num is divisible by both 5 and 3");
    } else if my_num % 5 == 0 {
        println!("my num is divisible by 5");
    } else if my_num % 3 == 0 {
        println!("my num is divisible by 3");
    } else {
        println!("My number is not divisible by 3 or 5");
    }

    // Using if in a let statement

    let condition: bool = true;
    let other_number: i32 = if condition { 5 } else { 6 };
    // if condition is true, it sets other_number to 5, otherwise, it sets other_number to 6

    println!("The value of other_number is: {other_number}");

    /*
    let some_num = if condition { 5 } else { "six" };
    println!("The value of some_num is: {some_num}");

    THIS WILL NOT WORK! Because... the types are mismatched

     */

    println!("--- LOOPS ---------------");

    /* This is an infinite loop that will continuously print again! to the terminal.
    loop {
        println!("again!");
    }
    */

    // We use breaks to escape the loop
    // In the example below, once counter reaches the value 10, we break and multiply counter by 2.

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of counter is: {result}");
    println!();

    // While loops

    let mut some_num = 3;

    while some_num != 0 {
        println!("{some_num}");

        some_num -= 1;
    }

    println!("LIFTOFF!");
    println!();

    // Looping through a collection with for loops
    //
    println!("WHILE LOOP THROUGH AN ARRAY\n");

    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is {}", arr[index]); // Prints value of arr at index
        index += 1; // Increments value of index by 1. You are looping through the array.
    }

    println!("\nBETTER FOR LOOP\n");

    // Better way to loop through an array
    let a = [10, 20, 30, 40, 50];

    // For each element in a, it prints the element.
    for element in a {
        println!("The value is: {element}");
    }
}
