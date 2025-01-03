fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(7, 'm');

    // Expressions
    // expressions do not have semicolons

    let y = {
        let x = 3;
        x+1
    };

    println!("The value of y is: {y}");


    let x = five();
    println!("The value of x is: {x}");

    let z = plus_one(8);
    println!("The value of z is: {z}");

    let someNumEven: bool = is_even(9);
    println!("Is 9 even? {someNumEven}");
}

fn another_function(x:i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, label: char){
    println!("The measurement is {value}{label}");
}


// Even though this func only contains the number 5, it is still perfectly legal. (even its return type is i32)
fn five() -> i32 {
    5 // 5 is an expression because it doesn't have a semicolon and this is the expression we want to return.
}

fn plus_one(x: i32) -> i32 {
    x+1
}

fn is_even(x:i32) -> bool {
    if x % 2 == 0 {
        return true;

    }
    else {
        return false;
    }
}