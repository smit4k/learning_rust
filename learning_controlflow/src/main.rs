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
}
