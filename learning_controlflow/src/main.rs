fn main() {
    let number:i32 = 3;


    if number < 5 {
        println!("condition was true");
    }
    else {
        println!("condition was false");
    }

    if number != 0 {
        println!("Number was something other than 0")
    }

    // else ifs
    let my_num: i32 = 45;

    if my_num % 5 == 0 && my_num % 3 == 0 {
        println!("my num is divisible by both 5 and 3");
    }
    else if my_num % 5 == 0 {
        println!("my num is divisible by 5");
    }
    else if my_num % 3 == 0 {
        println!("my num is divisible by 3");
    }
    else{
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
}
