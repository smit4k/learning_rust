fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(7, 'm');
}

fn another_function(x:i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, label: char){
    println!("The measurement is {value}{label}");
}
