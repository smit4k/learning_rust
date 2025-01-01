fn main() {
    let mut x: i8 = 5; // i32 is signed therefore it can be negative, unsigned int types do not have a sign (+ or -), therefore they can ONLY be positive(+)
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

    println!("-------------------------------------");

    // booleans 
    let t = true;
    let f: bool = false; // explicit type annotation

    println!("-------------------------------------");

    // chars

    // chars use single quotes '' & are rust's most primitive datatype

    let c = 'z';
    let z:char = 'Z'; // explicit type annotation
    let smiley_emoji = '😃';

    println!("{c}");
    println!("{z}");
    println!("{smiley_emoji}");
}
